pub mod error;
pub mod god;
pub mod ollama;

use god::GodConfig;
use once_cell::sync::Lazy;
use serenity::all::ComponentInteraction;
use serenity::gateway::ActivityData;
use std::env;
use std::fs;
use std::time::Duration;
use std::{collections::HashMap, sync::Arc};

use redis::Commands;

pub use crate::god::God;

use serenity::{
    async_trait,
    model::{channel::Message, gateway::Ready},
    prelude::{Client, Context, EventHandler, GatewayIntents, RwLock, TypeMapKey},
};

struct RedisClient {}
impl TypeMapKey for RedisClient {
    type Value = Arc<redis::Client>;
}

trait GodType {
    type God;
}
struct GodNursery;

impl TypeMapKey for GodNursery {
    type Value = RwLock<HashMap<u64, Arc<RwLock<God>>>>;
}
struct Handler {}

#[allow(dead_code)]
fn get_name<T>(_: T) -> String {
    return std::any::type_name::<T>().to_string();
}

const GOD_REQUEST: &str = "god: ";
const GOD_CLEAN: &str = "god clean";
const GOD_PRESENCE: &str = "god are you there?";
const GOD_ANY: &str = "god";
const GOD_CONFIG_SET: &str = "god set";
const GOD_CONFIG_GET: &str = "god get";
const MAX_MESSAGE_SIZE: usize = 8096;

const GOD_DEFAULT: Lazy<god::GodConfig> = Lazy::new(|| god::GodConfig::default());
static GOD_LIBRARY: Lazy<RwLock<HashMap<String, god::GodConfig>>> =
    Lazy::new(|| RwLock::new(HashMap::<String, god::GodConfig>::new()));

async fn get_or_create_bot(ctx: &Context, key: u64) -> Arc<RwLock<God>> {
    //ComponentInteraction
    // message_component::ComponentInteraction, modal::ModalInteraction, InteractionResponseType,
    let data = ctx.data.read().await;
    let nursery = data
        .get::<GodNursery>()
        .expect("There should be a nursery here.");

    let has_bot = nursery.read().await.contains_key(&key);

    if !has_bot {
        let client = data
            .get::<RedisClient>()
            .expect("There should be a redis client here.");

        let con = client.get_connection_with_timeout(Duration::from_secs(1));
        let new_god = match con {
            redis::RedisResult::Err(_error) => God::from_config(GOD_DEFAULT.clone()),
            redis::RedisResult::Ok(mut c) => {
                let result = c.get::<u64, String>(key);
                match result {
                    redis::RedisResult::Ok(val) => match God::import_json(val.as_str()) {
                        Some(god) => god,
                        _ => God::from_config(GOD_DEFAULT.clone()),
                    },
                    redis::RedisResult::Err(_error) => God::from_config(GOD_DEFAULT.clone()),
                }
            }
        };
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
        } else if lowercase == GOD_CONFIG_SET {
            if let Err(why) = msg.channel_id.say(&ctx.http, "This is under construction.").await {
                println!("Error sending message: {:?}", why);
            }
            // configure_god_mainmenu(&ctx, &msg, key).await;
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
        } else if lowercase.starts_with(GOD_REQUEST) {
            let prompt_slice = &msg.content[GOD_REQUEST.len()..];
            let author_name = msg.author.name.clone();
            let god = get_or_create_bot(&ctx, key.into()).await;

            let prompt = { god.read().await.get_prompt(&author_name, prompt_slice) };

            let response = { god.read().await.brain.request(&prompt).await };
            if let Some(response) = response {
                if let Err(why) = msg.channel_id.say(&ctx.http, &response.content).await {
                    println!("Error sending message: {:?}", why);
                }
                {
                    god.write().await.set_prompt_response(
                        &author_name,
                        prompt_slice,
                        &response.content,
                    );
                }
            }
        } else if lowercase.contains(GOD_ANY) || msg.is_private() {
            let prompt_slice = &msg.content[..];
            let author_name = msg.author.name.clone();

            let god = get_or_create_bot(&ctx, key.into()).await;

            let prompt = { god.read().await.get_prompt(&author_name, prompt_slice) };

            let response = { god.read().await.brain.request(&prompt).await };

            if let Some(response) = response {
                if let Err(why) = msg.channel_id.say(&ctx.http, &response.content).await {
                    println!("Error sending message: {:?}", why);
                }
                {
                    god.write().await.set_prompt_response(
                        &author_name,
                        prompt_slice,
                        &response.content,
                    );
                }
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

#[tokio::main]
async fn main() {
    // Configure the client with your Discord bot token in the environment.
    let token_discord =
        env::var("DISCORD_BOT_TOKEN").expect("Expected a DISCORD_BOT_TOKEN in the environment");
    let redis_uri = env::var("REDIS_URI").unwrap_or_else(|_| "redis://127.0.0.1/".to_string());
    let gods_path = env::var("GODS_PATH").unwrap_or_else(|_| "./gods".to_string());

    // Global god library preparation
    {
        let mut god_library = GOD_LIBRARY.write().await;
        let paths = fs::read_dir(gods_path).unwrap();
        for path in paths {
            let entry = if let Ok(data) = path {
                data
            } else {
                continue;
            };
            let path = entry.path();
            let should_be_read = if let Ok(data) = entry.metadata() {
                data.is_file()
                    && path.extension().unwrap_or_default()
                        == std::ffi::OsString::from("json").to_os_string()
            } else {
                false
            };

            if should_be_read {
                if let Ok(data) = fs::read_to_string(path) {
                    if let Ok(new_config) = serde_json::from_str::<GodConfig>(&data) {
                        god_library.insert(new_config.botname.clone(), new_config.clone());
                    }
                }
            }
        }
    }

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
        data.insert::<GodNursery>(RwLock::new(HashMap::default()));

        let library = GOD_LIBRARY.read().await;
        let god_library_values = library.values();

        /*
            let mut bot_ui = UI::default();
            bot_ui.build_load_config(god_library_values);
            data.insert::<UI>(bot_ui);
        */

        match redis::Client::open(redis_uri) {
            redis::RedisResult::Ok(client) => {
                println!("Redis client created");
                data.insert::<RedisClient>(Arc::new(client));
            }
            redis::RedisResult::Err(error) => {
                println!("Error connecting to Redis: {}", error);
            }
        }
    }

    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}
