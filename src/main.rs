pub mod ai21;
pub mod bot_ui;
pub mod error;
pub mod god;

use std::env;
use std::fs;
use std::time::Duration;
use std::{collections::HashMap, sync::Arc};
use once_cell::sync::Lazy;

use redis::Commands;

pub use crate::god::God;
use bot_ui::UI;
use serenity::builder::CreateComponents;
use serenity::model::interactions::message_component::ActionRowComponent;

use serenity::{
    async_trait,
    model::{
        channel::Message,
        gateway::Ready,
        interactions::{
            message_component::MessageComponentInteraction, modal::ModalSubmitInteraction,
            InteractionResponseType,
        },
    },
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

const  GOD_DEFAULT: Lazy<god::GodMemoryConfig> = Lazy::new(|| {
    god::GodMemoryConfig::default()
});
static GOD_LIBRARY: Lazy<RwLock<HashMap<String, god::GodMemoryConfig>>> = Lazy::new(|| {
    RwLock::new(HashMap::<String, god::GodMemoryConfig>::new())
});


async fn get_or_create_bot(ctx: &Context, key: u64) -> Arc<RwLock<God>> {
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
            redis::RedisResult::Err(_error) => God::from_config(&GOD_DEFAULT),
            redis::RedisResult::Ok(mut c) => {
                let result = c.get::<u64, String>(key);
                match result {
                    redis::RedisResult::Ok(val) => {
                        match God::import_json(val.as_str()) {
                            Some(god) => god,
                            _ => God::from_config(&GOD_DEFAULT),
                        }
                    }
                    redis::RedisResult::Err(_error) => God::from_config(&GOD_DEFAULT),
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

async fn request_modal_data(
    modal: CreateComponents,
    title: &str,
    ctx: &Context,
    mci: Arc<MessageComponentInteraction>,
) -> Option<Arc<ModalSubmitInteraction>> {
    mci.create_interaction_response(&ctx, |r| {
        r.kind(InteractionResponseType::Modal)
            .interaction_response_data(|d| {
                d.custom_id("modal_data").set_components(modal).title(title)
            })
    })
    .await
    .unwrap();
    mci.message.await_modal_interaction(&ctx.shard).await
}

async fn change_name(ctx: &Context, mci: Arc<MessageComponentInteraction>, key: u64) {
    let data = ctx.data.read().await;
    let ui = data.get::<UI>().expect("There should be a UI here.");

    let modal_collector =
        request_modal_data(ui.get_change_name(), "Change god name", ctx, mci.clone()).await;

    let modal = match modal_collector {
        Some(modal) => modal,
        None => {
            mci.message.reply(&ctx, "Timed out").await.unwrap();
            return;
        }
    };

    modal
        .create_interaction_response(ctx.http.clone(), |f| {
            f.kind(InteractionResponseType::UpdateMessage)
                .interaction_response_data(|d| d.content("Gocha!"))
        })
        .await
        .unwrap();

    match &modal.data.components[0].components[0] {
        ActionRowComponent::InputText(input_text) => {
            {
                let god = get_or_create_bot(ctx, key).await;
                god.write().await.set_botname(&input_text.value);
            }
            mci.message
                .reply(&ctx, format!("I am now named {}", input_text.value))
                .await
                .unwrap();
        }
        _ => {
            mci.message
                .reply(&ctx, "Please do not break my god.")
                .await
                .unwrap();
        }
    }
}

async fn change_context(ctx: &Context, mci: Arc<MessageComponentInteraction>, key: u64) {
    let data = ctx.data.read().await;
    let ui = data.get::<UI>().expect("There should be a UI here.");

    let modal_collector =
        request_modal_data(ui.get_change_context(), "Change context", ctx, mci.clone()).await;

    let modal = match modal_collector {
        Some(modal) => modal,
        None => {
            mci.message.reply(&ctx, "Timed out").await.unwrap();
            return;
        }
    };

    modal
        .create_interaction_response(ctx.http.clone(), |f| {
            f.kind(InteractionResponseType::UpdateMessage)
                .interaction_response_data(|d| d.content("Gocha!"))
        })
        .await
        .unwrap();

    match &modal.data.components[0].components[0] {
        ActionRowComponent::InputText(input_text) => {
            {
                let god = get_or_create_bot(ctx, key).await;
                god.write().await.set_context(&input_text.value);
            }
            mci.message
                .reply(&ctx, format!("My new context is {}", input_text.value))
                .await
                .unwrap();
        }
        _ => {
            mci.message
                .reply(&ctx, "Please do not break my god.")
                .await
                .unwrap();
        }
    }
}

async fn add_interaction(ctx: &Context, mci: Arc<MessageComponentInteraction>, key: u64) {
    let data = ctx.data.read().await;
    let ui = data.get::<UI>().expect("There should be a UI here.");

    let modal_collector = request_modal_data(
        ui.get_add_interaction(),
        "Add an interaction",
        ctx,
        mci.clone(),
    )
    .await;

    let modal = match modal_collector {
        Some(modal) => modal,
        None => {
            mci.message.reply(&ctx, "Timed out").await.unwrap();
            return;
        }
    };

    modal
        .create_interaction_response(ctx.http.clone(), |f| {
            f.kind(InteractionResponseType::UpdateMessage)
                .interaction_response_data(|d| d.content("Gocha!"))
        })
        .await
        .unwrap();

    let author = match &modal.data.components[0].components[0] {
        ActionRowComponent::InputText(input_text) => input_text.value.as_str(),
        _ => "",
    };
    let prompt = match &modal.data.components[1].components[0] {
        ActionRowComponent::InputText(input_text) => input_text.value.as_str(),
        _ => "",
    };
    let response = match &modal.data.components[2].components[0] {
        ActionRowComponent::InputText(input_text) => input_text.value.as_str(),
        _ => "",
    };

    if author.is_empty() || prompt.is_empty() || response.is_empty() {
        mci.message
            .reply(ctx.http.clone(), "One of the inputs is empty.")
            .await
            .unwrap();
    } else {
        {
            let god = get_or_create_bot(ctx, key).await;
            god.write().await.add_interaction(author, prompt, response);
        }
        mci.message
            .reply(ctx.http.clone(), "New default interaction added!")
            .await
            .unwrap();
    }
}

async fn clear_interactions(ctx: &Context, mci: Arc<MessageComponentInteraction>, key: u64) {
    {
        let god = get_or_create_bot(ctx, key).await;
        god.write().await.clear_interactions();
    }
    mci.message
        .reply(ctx.http.clone(), "All interactions have been removed.")
        .await
        .unwrap();
}

async fn save(ctx: &Context, mci: Arc<MessageComponentInteraction>, key: u64) {
    let data = ctx.data.read().await;
    let client = data
        .get::<RedisClient>()
        .expect("There should be a redis client here.");

    let con = client.get_connection_with_timeout(Duration::from_secs(1));
    match con {
        redis::RedisResult::Err(e) => println!("Failed to connect: {}", e),
        redis::RedisResult::Ok(mut c) => {
            let god = get_or_create_bot(ctx, key).await;
            let json = god.write().await.export_json();
            let result = c.set::<u64, String, ()>(key, json.to_string());
            match result {
                redis::RedisResult::Ok(()) => println!("Success"),
                redis::RedisResult::Err(e) => println!("Error: {}", e),
            }
        }
    }
    mci.message
        .reply(ctx.http.clone(), "This might be saved at some point.")
        .await
        .unwrap();
}

async fn configure_god_mainmenu(ctx: &Context, msg: &Message, key: u64) {
    let data = ctx.data.read().await;
    let ui = data.get::<UI>().expect("There should be a UI here.");

    let m = msg
        .channel_id
        .send_message(&ctx, |m| {
            m.content("Please select your configuration")
                .set_components(ui.get_main_menu())
            //    .components(|c| c.add_action_row(action_select))
        })
        .await
        .unwrap();

    let mci = match m
        .await_component_interaction(&ctx)
        .timeout(Duration::from_secs(60 * 3))
        .await
    {
        Some(ci) => ci,
        None => {
            m.reply(&ctx, "Timed out").await.unwrap();
            return;
        }
    };

    let response = mci.data.custom_id.clone();

    match response.as_str() {
        "change_name" => change_name(ctx, mci.clone(), key).await,
        "change_context" => change_context(ctx, mci.clone(), key).await,
        "add_interaction" => add_interaction(ctx, mci.clone(), key).await,
        "clear_interactions" => clear_interactions(ctx, mci.clone(), key).await,
        "save" => save(ctx, mci.clone(), key).await,
        _ => {}
    }
    m.delete(&ctx).await.unwrap();
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

        if lowercase == GOD_CLEAN {
            let god = get_or_create_bot(&ctx, key).await;
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
            configure_god_mainmenu(&ctx, &msg, key).await;
        } else if lowercase == GOD_CONFIG_GET {
            let god = get_or_create_bot(&ctx, key).await;
            let config = god.read().await.get_config();
            if let Err(why) = msg.channel_id.say(&ctx.http, &config).await {
                println!("Error sending message: {:?}", why);
            }
        } else if lowercase.starts_with(GOD_REQUEST) {
            let prompt_slice = &msg.content[GOD_REQUEST.len()..];
            let author_name = msg.author.name.clone();

            let god = get_or_create_bot(&ctx, key).await;

            let prompt = { god.read().await.get_prompt(&author_name, prompt_slice) };

            let response = { god.read().await.brain.request(&prompt).await };

            if !response.is_empty() {
                if let Err(why) = msg.channel_id.say(&ctx.http, &response).await {
                    println!("Error sending message: {:?}", why);
                }
                {
                    god.write()
                        .await
                        .set_prompt_response(&author_name, prompt_slice, &response);
                }
            }
        } else if lowercase.contains(GOD_ANY) || msg.is_private() {
            let prompt_slice = &msg.content[..];
            let author_name = msg.author.name.clone();

            let god = get_or_create_bot(&ctx, key).await;

            let prompt = { god.read().await.get_prompt(&author_name, prompt_slice) };

            let response = { god.read().await.brain.request(&prompt).await };

            if !response.is_empty() {
                if let Err(why) = msg.channel_id.say(&ctx.http, &response).await {
                    println!("Error sending message: {:?}", why);
                }
                {
                    god.write()
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
    let redis_uri = env::var("REDIS_URI").unwrap_or_else(|_| "redis://127.0.0.1/".to_string());
    let gods_path = env::var("GODS_PATH").unwrap_or_else(|_| "./gods".to_string());

    // Global god library preparation
    {
        let mut god_library = GOD_LIBRARY.write().await;
        let paths = fs::read_dir(gods_path).unwrap();
        for path in paths {
            let entry = if let Ok(data) = path { data } else {continue;};
            let path = entry.path();
            let should_be_read = if let Ok(data) = entry.metadata() {
                data.is_file() && path.extension().unwrap_or_default() == std::ffi::OsString::from("json").to_os_string()
            } else { false };

            if should_be_read {
                dbg!(entry.path());
            }

            if let Ok(data) = fs::read_to_string(path) {
                if let Ok(new_config) = serde_json::from_str::<god::GodMemoryConfig>(&data) {
                    god_library.insert(new_config.botname.clone(), new_config.clone());
                }
            }
        }
        dbg!(god_library);
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
        data.insert::<UI>(UI::default());

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
