pub mod ai21;
pub mod kirby;
pub mod error;

use std::{collections::HashMap, sync::Arc};

pub use crate::kirby::Kirby;

use std::env;

use serenity::{
    async_trait,
    model::{channel::Message, gateway::Ready},
    prelude::{Client, Context, EventHandler, RwLock, TypeMapKey},
};

struct KirbyNursery;

impl TypeMapKey for KirbyNursery {
    type Value = RwLock<HashMap<u64, Arc<RwLock<Kirby>>>>;
}

struct Handler {}

fn get_name<T>(_: T) -> String {
    return std::any::type_name::<T>().to_string();
}

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        let key = msg.channel_id.0.clone();

        if msg.content.starts_with("kirby clean") {
            let data = ctx.data.read().await;
            let nursery = data
                .get::<KirbyNursery>()
                .expect("There should be a nursery here.")
                .clone();

            let has_kirby = nursery.read().await.contains_key(&key);

            if has_kirby {
                let kirby = {
                    let read_nursery = nursery.read().await;
                    read_nursery.get(&key).unwrap().clone()
                };
                kirby.write().await.clear();
            }
        }

        if msg.content.starts_with("OMG") {
            let prompt_slice = &msg.content["OMG".len()..];
            let author_name = msg.author.name.clone();

            let data = ctx.data.read().await;
            let nursery = data
                .get::<KirbyNursery>()
                .expect("There should be a nursery here.");

            let has_kirby = nursery.read().await.contains_key(&key);

            if !has_kirby {
                let mut write_nursery = nursery.write().await;
                write_nursery.insert(key, Arc::new(RwLock::new(Kirby::new("Kirby"))));
            }

            let kirby = {
                let read_nursery = nursery.read().await;
                read_nursery.get(&key).unwrap().clone()
            };

            let prompt = { kirby.write().await.get_prompt(&author_name, prompt_slice) };

            let response = { kirby.read().await.brain.request(&prompt).await };

            if response != "" {
                if let Err(why) = msg.channel_id.say(&ctx.http, &response).await {
                    println!("Error sending message: {:?}", why);
                }
                kirby.write().await.set_response(&response);
            }
        }
    }

    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}

#[tokio::main]
async fn main() {
    // Configure the client with your Discord bot token in the environment.
    let token_discord =
        env::var("DISCORD_BOT_TOKEN").expect("Expected a token in the environment for discord");
    let mut client = Client::builder(&token_discord)
        .event_handler(Handler {})
        .await
        .expect("Err creating client");

    // Quick test of the prompts
    let initial_prompt: kirby::Discussion = kirby::Discussion(
        vec![
            kirby::DiscussionKind::Prompt{
                author: "Alexis".to_string(),
                prompt: "Who is god?".to_string()
            },
            kirby::DiscussionKind::Response{
                author: "Kirby".to_string(),
                prompt: "Well, now that you ask, I can tell you. I, Kirby is the great goddess is the god of everybody!".to_string()
            }],
    );
    let mut memory = kirby::AIMemory::new(String::from("This is Kirby... LoL"), initial_prompt);
    let _prompt = memory.get_prompt("Alexis", "Coucou", "Kirby");
    memory.set_response("Kirby", "Oh, okay");
    println!("{}", memory.to_string());

    // Prepare the Kirby nursery global data
    {
        let mut data = client.data.write().await;
        data.insert::<KirbyNursery>(RwLock::new(HashMap::default()));
    }

    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}
