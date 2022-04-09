pub mod ai21;
pub mod kirby;

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
    type Value = HashMap<String, Kirby>;
}

struct Handler {}

fn get_name<T>(val: T) -> String {
    return std::any::type_name::<T>().to_string();
}

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.content.starts_with("OMG") {
            let prompt_slice = &msg.content["OMG".len()..];
            let author_name = msg.author.name.clone();
            {
                let mut data = ctx.data.write().await;
                let nursery = data
                    .get_mut::<KirbyNursery>()
                    .expect("There should be a nursery here.");

                let kirby = nursery
                    .entry("hey".to_string())
                    .or_insert(Kirby::new("Kirby"));

                let prompt = kirby.get_prompt(&author_name, prompt_slice);

                println!("Prompt: {:?}", prompt);
                let res = kirby.brain.request(&prompt).await;

                //prompt_slice.to_string(), res, author_name, "Kirby".to_string()
                if res != "" {
                    if let Err(why) = msg.channel_id.say(&ctx.http, &res).await {
                        println!("Error sending message: {:?}", why);
                    }
                    kirby.set_response(&res);
                }

                if let Err(why) = msg
                    .channel_id
                    .say(&ctx.http, &kirby.memory.to_string())
                    .await
                {
                    println!("Error sending message: {:?}", why);
                }
            };
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
        vec![kirby::DiscussionKind::Prompt{author: "Alexis".to_string(), prompt: "Who is god?".to_string()}, kirby::DiscussionKind::Response{author: "Kirby".to_string(), prompt: "Well, now that you ask, I can tell you. I, Kirby is the great goddess is the god of everybody!".to_string()}],
    );
    let mut memory = kirby::AIMemory::new(String::from("This is Kirby... LoL"), initial_prompt);
    let _prompt = memory.get_prompt("Alexis", "Coucou", "Kirby");
    memory.set_response("Kirby", "Oh, okay");
    println!("{}", memory.to_string());

    // Prepare the Kirby nursery global data
    {
        let mut data = client.data.write().await;
        data.insert::<KirbyNursery>(HashMap::default());
    }

    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}
