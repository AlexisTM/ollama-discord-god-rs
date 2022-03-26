mod ai21;
mod kirby;
pub use crate::ai21::request;
pub use crate::kirby::Kirby;

use std::env;

use serenity::{
    async_trait,
    model::{channel::Message, gateway::Ready},
    prelude::*,
};

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.content.starts_with("OMG") {
            let separators = "['.']";
            let token_ai21 =
                env::var("GOD_AI21_TOKEN").expect("Expected a token in the environment for AI21");
            let resp = ai21::request(
                &token_ai21,
                &msg.content,
                Some(8),
                &separators,
                Some(0.8),
                Some(1.0),
                Some(1),
                Some(1),
            )
            .await;

            match resp {
                Ok(n) => {
                    if let Err(why) = msg.channel_id.say(&ctx.http, &n).await {
                        println!("Error sending message: {:?}", why);
                    }
                }
                Err(error) => println!("Error getting the response {}", error),
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
        .event_handler(Handler)
        .await
        .expect("Err creating client");

    let a = kirby::AIPromptResponse{ prompt: "hey?".to_string(), response: "hahaha".to_string(), author: "Alexis".to_string(), botname: "Kirby".to_string() };
    let mut b = kirby::AIMemory::new(String::from("This is Kirby... LoL"), a);
    b.update(String::from("What is your favourite dish?"), String::from("Fish."), String::from("Alexis"),  String::from("Kirby"));
    println!("{}", b.to_string());

    if let Err(why) = client.start_shards(2).await {
        println!("Client error: {:?}", why);
    }
}
