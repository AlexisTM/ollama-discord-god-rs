pub mod ai21;
pub mod kirby;

use std::{collections::HashMap, sync::Arc};

pub use crate::kirby::{AIPromptResponse, Kirby};

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
            let slice = &msg.content["OMG".len()..];
            let author_name = msg.author.name.clone();
            let possible_prompt = AIPromptResponse {
                prompt: slice.to_string(),
                response: "".to_string(),
                author: author_name.clone(),
                botname: "Kirby".to_string(),
            };

            {
                let mut data = ctx.data.write().await;
                let nursery = data
                    .get_mut::<KirbyNursery>()
                    .expect("There should be a nursery here.");

                // .clone(); // ["val"]; //HashMap::<String, RwLock<Kirby>>::new()));
                //alloc::sync::Arc<std::collections::hash::map::HashMap<alloc::string::String, tokio::sync::rwlock::RwLock<kirby::kirby::Kirby>>>
                // println!("{}", get_name(nursery));

                let kirby = nursery.entry("hey".to_string()).or_insert(Kirby::new());

                let prompt = format!(
                    "{}\n{}",
                    kirby.memory.to_string(),
                    possible_prompt.to_string()
                );
                println!("Prompt: {:?}", prompt);
                let res = kirby.brain.request(&prompt).await;

                if res != "" {
                    if let Err(why) = msg.channel_id.say(&ctx.http, &res).await {
                        println!("Error sending message: {:?}", why);
                    }
                    kirby
                        .memory
                        .update(slice.to_string(), res, author_name, "Kirby".to_string());
                }

                if let Err(why) = msg
                    .channel_id
                    .say(&ctx.http, &kirby.memory.to_string())
                    .await
                {
                    println!("Error sending message: {:?}", why);
                }
            };


            /*
            let prompt = format!(
                "{}\n{}",
                self.kirby.memory.to_string(),
                possible_prompt.to_string()
            );
            println!("Prompt: {:?}", prompt);
            let res = self.kirby.brain.request(&prompt).await;

            if res != "" {
                if let Err(why) = msg.channel_id.say(&ctx.http, &res).await {
                    println!("Error sending message: {:?}", why);
                }
                self.kirby
                    .memory
                    .update(slice.to_string(), res, author_name, "Kirby".to_string());
            }
            /*
            let separators: Vec<String> = vec![".".to_string()];
            let token_ai21 =
                env::var("GOD_AI21_TOKEN").expect("Expected a token in the environment for AI21");
            let resp = ai21::request(
                &token_ai21,
                &msg.content,
                8,
                &separators,
                0.8,
                1.0
            )
            .await;
            */
            match resp {
                Ok(n) => {
                    if let Err(why) = msg.channel_id.say(&ctx.http, &n).await {
                        println!("Error sending message: {:?}", why);
                    }
                }
                Err(error) => println!("Error getting the response {}", error),
            }
            */
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

    let a = kirby::AIPromptResponse {
        prompt: "hey?".to_string(),
        response: "hahaha".to_string(),
        author: "Alexis".to_string(),
        botname: "Kirby".to_string(),
    };
    let mut b = kirby::AIMemory::new(String::from("This is Kirby... LoL"), a);
    b.update(
        String::from("What is your favourite dish?"),
        String::from("Fish."),
        String::from("Alexis"),
        String::from("Kirby"),
    );

    {
        // Open the data lock in write mode, so keys can be inserted to it.
        let mut data = client.data.write().await;

        // The CommandCounter Value has the following type:
        // Arc<RwLock<HashMap<String, u64>>>
        // So, we have to insert the same type to it.
        // Arc<HashMap<String, RwLock<Kirby>>>
        data.insert::<KirbyNursery>(HashMap::default());

        // data.insert::<MessageCount>(Arc::new(AtomicUsize::new(0)));
    }
    println!("{}", b.to_string());

    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}
