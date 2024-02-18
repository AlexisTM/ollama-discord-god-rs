use clap::Parser;

pub mod god;
pub mod ollama;

use god::{GodConfig, GodNursery};

use serenity::gateway::ActivityData;
use std::env;
use std::fs;
use std::{collections::HashMap, sync::Arc};

pub use crate::god::God;

use serenity::{
    async_trait,
    model::{channel::Message, gateway::Ready},
    prelude::{Client, Context, EventHandler, GatewayIntents, RwLock},
};

struct Handler {}

#[allow(dead_code)]
fn get_name<T>(_: T) -> String {
    return std::any::type_name::<T>().to_string();
}

const GOD_CLEAN: &str = "god clean";
const GOD_CONFIG_GET: &str = "god get";
const GOD_PRESENCE: &str = "god are you there?";
const MAX_MESSAGE_SIZE: usize = 8096;

async fn get_or_create_bot(ctx: &Context, key: u64) -> Arc<RwLock<God>> {
    //ComponentInteraction
    // message_component::ComponentInteraction, modal::ModalInteraction, InteractionResponseType,
    let data = ctx.data.read().await;
    let nursery = data
        .get::<GodNursery>()
        .expect("There should be a nursery here.");

    let default_god = data
        .get::<GodConfig>()
        .expect("There should be a default config in the context.");

    let has_bot = nursery.read().await.contains_key(&key);

    if !has_bot {
        let new_god = God::from_config(default_god.clone());
        let mut write_nursery = nursery.write().await;
        write_nursery.insert(key, Arc::new(RwLock::new(new_god)));
    }

    let bot = {
        let read_nursery = nursery.read().await;
        read_nursery.get(&key).unwrap().clone()
    };

    bot
}

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        // Prevent answering itself.
        let bot_user = ctx.http.get_current_user().await;

        let val = match bot_user {
            Ok(val) => Some(val.id),
            Err(_) => None,
        };

        if val == None || val.unwrap() == msg.author.id {
            return;
        }

        let key = msg.channel_id;
        let lowercase = msg.content.to_ascii_lowercase();

        if lowercase == GOD_CLEAN {
            let god = get_or_create_bot(&ctx, key.into()).await;
            god.write().await.clear();
            if let Err(why) = msg
                .channel_id
                .say(
                    &ctx.http,
                    "Oh, right, I just forgot about this whole thing.",
                )
                .await
            {
                println!("Error sending message: {:?}", why);
            }
        } else if lowercase.starts_with(GOD_PRESENCE) {
            if let Err(why) = msg.channel_id.say(&ctx.http, "Yes.").await {
                println!("Error sending message: {:?}", why);
            }
        } else if lowercase == GOD_CONFIG_GET {
            let god = get_or_create_bot(&ctx, key.into()).await;
            let config = god.read().await.get_config();
            let config_size = config.len();
            let mut config_curr = 0;

            while config_curr + MAX_MESSAGE_SIZE < config_size {
                let config_next = config_curr + MAX_MESSAGE_SIZE;
                if let Err(why) = msg
                    .channel_id
                    .say(&ctx.http, &config[config_curr..config_next])
                    .await
                {
                    println!("Error sending message: {:?}", why);
                }
                config_curr = config_next;
            }

            if let Err(why) = msg
                .channel_id
                .say(&ctx.http, &config[config_curr..config_size])
                .await
            {
                println!("Error sending message: {:?}", why);
            }
        }
    }

    async fn ready(&self, context: Context, _: Ready) {
        use serenity::model::user::OnlineStatus;
        let activity = ActivityData::watching("the world burn");
        let status = OnlineStatus::DoNotDisturb;
        context.set_presence(Some(activity), status);
    }
}

#[derive(Parser)]
struct Cli {
    pub god: std::path::PathBuf,
}

#[tokio::main]
async fn main() {
    // Configure the client with your Discord bot token in the environment.
    let token_discord =
        env::var("DISCORD_BOT_TOKEN").expect("Expected a DISCORD_BOT_TOKEN in the environment");

    let args = Cli::parse();

    println!("Reading: {:?}", args.god);

    let god_data: String = fs::read_to_string(&args.god)
        .expect(format!("The god {:?} file must be readable.", &args.god).as_str());
    let config = match serde_json::from_str::<GodConfig>(&god_data) {
        Ok(config) => Some(config),
        Err(err) => {
            println!("Parsing failed: {err}");
            None
        }
    }
    .expect("The god config should be valid.");

    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT
        | GatewayIntents::GUILD_INTEGRATIONS;

    let mut client = Client::builder(&token_discord, intents)
        .event_handler(Handler {})
        .await
        .expect("Err creating client");

    {
        let mut data = client.data.write().await;
        data.insert::<GodConfig>(config);
        data.insert::<GodNursery>(RwLock::new(HashMap::default()));
    }

    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}
