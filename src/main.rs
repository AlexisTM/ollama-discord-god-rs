pub mod ai21;
pub mod error;
pub mod kirby;

use std::{collections::HashMap, sync::Arc};

pub use crate::kirby::Kirby;
use const_format::formatcp;

use std::env;

use serenity::{
    async_trait,
    model::{
        channel::{ChannelType, Message},
        gateway::{Gateway, Ready},
        interactions::{self, Interaction},
    },
    prelude::{Client, Context, EventHandler, GatewayIntents, RwLock, TypeMapKey},
};

trait GodType {
    type God;
}

struct GodNursery<T>(T); //{}

impl<T> GodType for GodNursery<T> {
    type God = T;
}

impl<T: Sync + Send + 'static> TypeMapKey for GodNursery<T> {
    type Value = RwLock<HashMap<u64, Arc<RwLock<T>>>>;
}

struct KirbyNursery;

impl TypeMapKey for KirbyNursery {
    type Value = RwLock<HashMap<u64, Arc<RwLock<Kirby>>>>;
}
struct Handler {}

fn get_name<T>(_: T) -> String {
    return std::any::type_name::<T>().to_string();
}

const KIRBY_REQUEST: &str = "kirby: ";
const KIRBY_CLEAN: &str = "kirby clean";
const KIRBY_PRESENCE: &str = "kirby are you there?";
const KIRBY_ANY: &str = "kirby";

const KIRBY_CONFIG_GET: &str = "kirby config get";
const KIRBY_CONFIG_SET_CONTEXT: &str = "kirby context set";
const KIRBY_CONFIG_SET_NAME: &str = "kirby name set";
const KIRBY_CONFIG_ADD_INTERACTION: &str = "kirby interaction add";
const KIRBY_CONFIG_ADD_INTERACTION_INFO: &str = "Usage: kirby interaction add
---
AlexisTM
---
But what is the meaning of Life, Kirby?
---
Actually, it is pretty interesting. It is to be gobbed by myself so I can become the better version of you.";
const KIRBY_CONFIG_CLEAR_INTERACTIONS: &str = "kirby interaction clear\n";
const KIRBY_CONFIG_SAVE: &str = "kirby save";
const KIRBY_CONFIG: &str = "kirby config";

const KIRBY_CONFIG_STR: &str = formatcp!(
    "Kirby commands
===============
{KIRBY_CONFIG} => This note
{KIRBY_CONFIG_GET} => Returns current config
{KIRBY_CONFIG_SET_NAME} => Sets the name of the bot
{KIRBY_CONFIG_SET_CONTEXT} => Sets the context
{KIRBY_CONFIG_ADD_INTERACTION} => Adds a default request/repsonse in the initial memory
{KIRBY_CONFIG_CLEAR_INTERACTIONS} => Removes the initial memorys
{KIRBY_CONFIG_SAVE} => Saves the current kirby");

async fn get_or_create_bot(ctx: &Context, key: u64) -> Arc<RwLock<Kirby>> {
    let data = ctx.data.read().await;
    let nursery = data
        .get::<KirbyNursery>()
        .expect("There should be a nursery here.");

    let has_bot = nursery.read().await.contains_key(&key);

    if !has_bot {
        let mut write_nursery = nursery.write().await;
        write_nursery.insert(key, Arc::new(RwLock::new(Kirby::new("Kirby"))));
    }

    let bot = {
        let read_nursery = nursery.read().await;
        read_nursery.get(&key).unwrap().clone()
    };

    return bot;
}

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        // Prevent answering itself.
        let bot_user = ctx.http.get_current_user().await;

        let val = match bot_user {
            Ok(val) => val.id,
            Err(_) => serenity::model::id::UserId(0),
        };

        if val == serenity::model::id::UserId(0) || val == msg.author.id {
            return;
        }

        let key = msg.channel_id.0;
        let lowercase = msg.content.to_ascii_lowercase();

        if lowercase.starts_with(KIRBY_CLEAN) {
            let kirby = get_or_create_bot(&ctx, key).await;
            kirby.write().await.clear();
        } else if lowercase.starts_with(KIRBY_PRESENCE) {
            if let Err(why) = msg.channel_id.say(&ctx.http, "Yes.").await {
                println!("Error sending message: {:?}", why);
            }
        } else if lowercase.starts_with(KIRBY_CONFIG_GET) {
            let kirby = get_or_create_bot(&ctx, key).await;
            let config = kirby.read().await.get_config();
            if let Err(why) = msg.channel_id.say(&ctx.http, &config).await {
                println!("Error sending message: {:?}", why);
            }
        } else if lowercase.starts_with(KIRBY_CONFIG_SET_CONTEXT) {
            let new_context = &msg.content[KIRBY_CONFIG_SET_CONTEXT.len()..];
            let kirby = get_or_create_bot(&ctx, key).await;
            kirby.write().await.set_context(new_context);
            let feedback = format!("New context:\n----------\n{}", new_context);
            if let Err(why) = msg.channel_id.say(&ctx.http, feedback).await {
                println!("Error sending message: {:?}", why);
            }
        } else if lowercase.starts_with(KIRBY_CONFIG_SET_NAME) {
            let name = &msg.content[KIRBY_CONFIG_SET_NAME.len()..];
            let kirby = get_or_create_bot(&ctx, key).await;
            kirby.write().await.set_botname(name);
            let feedback = format!("New name:\n----------\n{}", name);
            if let Err(why) = msg.channel_id.say(&ctx.http, feedback).await {
                println!("Error sending message: {:?}", why);
            }
        } else if lowercase.starts_with(KIRBY_CONFIG_CLEAR_INTERACTIONS) {
            let kirby = get_or_create_bot(&ctx, key).await;
            kirby.write().await.clear_interactions();
            let feedback = format!("All interactions have been removed.");
            if let Err(why) = msg.channel_id.say(&ctx.http, feedback).await {
                println!("Error sending message: {:?}", why);
            }
        } else if lowercase.starts_with(KIRBY_CONFIG_ADD_INTERACTION) {
            // TODO: Save to server
            let interaction = &msg.content[KIRBY_CONFIG_ADD_INTERACTION.len()..];
            let interactions = interaction.split("\n---\n").collect::<Vec<&str>>();
            if interactions.len() != 4 {
                if let Err(why) = msg
                    .channel_id
                    .say(&ctx.http, KIRBY_CONFIG_ADD_INTERACTION_INFO)
                    .await
                {
                    println!("Error sending message: {:?}", why);
                }
                return;
            }
            let kirby = get_or_create_bot(&ctx, key).await;
            kirby
                .write()
                .await
                .add_interaction(interactions[1], interactions[2], interactions[3]);
            let feedback = format!("New interaction added.");
            if let Err(why) = msg.channel_id.say(&ctx.http, feedback).await {
                println!("Error sending message: {:?}", why);
            }
        } else if lowercase.starts_with(KIRBY_CONFIG_SAVE) {
            // TODO: Permanent save to redis?
            if let Err(why) = msg.channel_id.say(&ctx.http,
                    "I wish I could save it to redis. Wait for that, or DIY and make a quick PR. Appreciated ;)").await {
                println!("Error sending message: {:?}", why);
            }
        } else if lowercase == KIRBY_CONFIG {
            if let Err(why) = msg.channel_id.say(&ctx.http, KIRBY_CONFIG_STR).await {
                println!("Error sending message: {:?}", why);
            }
        } else if lowercase.starts_with(KIRBY_REQUEST) {
            let prompt_slice = &msg.content[KIRBY_REQUEST.len()..];
            let author_name = msg.author.name.clone();

            let kirby = get_or_create_bot(&ctx, key).await;

            let prompt = { kirby.read().await.get_prompt(&author_name, prompt_slice) };

            let response = { kirby.read().await.brain.request(&prompt).await };

            if !response.is_empty() {
                if let Err(why) = msg.channel_id.say(&ctx.http, &response).await {
                    println!("Error sending message: {:?}", why);
                }
                {
                    kirby
                        .write()
                        .await
                        .set_prompt_response(&author_name, prompt_slice, &response);
                }
            }
        } else if lowercase.contains(KIRBY_ANY) || msg.is_private() {
            let prompt_slice = &msg.content[..];
            let author_name = msg.author.name.clone();

            let kirby = get_or_create_bot(&ctx, key).await;

            let prompt = { kirby.read().await.get_prompt(&author_name, prompt_slice) };

            let response = { kirby.read().await.brain.request(&prompt).await };

            if !response.is_empty() {
                if let Err(why) = msg.channel_id.say(&ctx.http, &response).await {
                    println!("Error sending message: {:?}", why);
                }
                {
                    kirby
                        .write()
                        .await
                        .set_prompt_response(&author_name, prompt_slice, &response);
                }
            }
        }
    }

    async fn ready(&self, context: Context, _: Ready) {
        use serenity::model::gateway::Activity;
        use serenity::model::user::OnlineStatus;

        let activity = Activity::playing("Being the master of the universe.");
        let status = OnlineStatus::DoNotDisturb;

        context.set_presence(Some(activity), status).await;
    }
}

#[tokio::main]
async fn main() {
    // Configure the client with your Discord bot token in the environment.
    let token_discord =
        env::var("DISCORD_BOT_TOKEN").expect("Expected a token in the environment for discord");

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
        data.insert::<KirbyNursery>(RwLock::new(HashMap::default()));
    }

    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}
