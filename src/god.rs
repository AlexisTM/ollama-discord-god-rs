use ollama_rs::generation::chat::MessageRole;
use ollama_rs::generation::options::GenerationOptions;
use serde::{Deserialize, Serialize};
use serde_json::json;
use serenity::prelude::{RwLock, TypeMapKey};

use crate::ollama::OllamaAI;
use ollama_rs::generation::chat::ChatMessage;
use std::clone::Clone;
use std::collections::HashMap;
use std::fmt;
use std::sync::Arc;

const MAX_RECOLLECTIONS: usize = 10;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum DiscussionKind {
    // Splitting them allows to put some different parsing (one extra \n) for responses.
    // Another implementation would have been to use a NewLine type and have only Prompts.
    Prompt { author: String, prompt: String },
    Response { author: String, prompt: String },
}

impl fmt::Display for DiscussionKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DiscussionKind::Prompt { author, prompt } => {
                writeln!(f, "{}: {}", author, prompt)
            }
            DiscussionKind::Response { author, prompt } => {
                writeln!(f, "{}: {}\n\n---", author, prompt)
            }
        }
    }
}

pub struct GodNursery;
impl TypeMapKey for GodNursery {
    type Value = RwLock<HashMap<u64, Arc<RwLock<God>>>>;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GodConfig {
    // We were created last thursday, this is the discussion the bot is born with.
    pub model: String,
    pub botname: String,
    pub options: GenerationOptions,
    // We were created last thursday, this is the discussion the bot is born with.
    pub thursdayism: Vec<ChatMessage>,
}

impl TypeMapKey for GodConfig {
    type Value = GodConfig;
}

impl Default for GodConfig {
    fn default() -> Self {
        let thursdayism: Vec<ChatMessage> = vec![
            ChatMessage::new(
                MessageRole::System,
                "Alexis: Oh! Look there! What is that?".to_owned(),
            ),
            ChatMessage::new(
                MessageRole::User,
                "Alexis: Oh! Look there! What is that?".to_owned(),
            ),
            ChatMessage::new(
                MessageRole::Assistant,
                "Oh, that is king Dedede! I'm soooo scared!".to_owned(),
            ),
            ChatMessage::new(
                MessageRole::User,
                "Alexis: Let's fight this ennemy!".to_owned(),
            ),
            ChatMessage::new(MessageRole::Assistant, "But i have no sword!?!".to_owned()),
            ChatMessage::new(
                MessageRole::User,
                "Alexis: Here, take this minion.".to_owned(),
            ),
            ChatMessage::new(
                MessageRole::Assistant,
                "Oof! Thanks for that! I can now fight!".to_owned(),
            ),
        ];
        let options = GenerationOptions::default()
            .num_ctx(4096)
            .num_predict(256)
            .temperature(0.8)
            .top_k(40)
            .top_p(0.9)
            .num_gpu(100)
            .num_thread(4);

        Self {
            model: "mistral".to_owned(),
            botname: "Kirby".to_owned(),
            options,
            thursdayism,
        }
    }
}

// trait Bot, for God
#[derive(Debug)]
pub struct God {
    pub brain: OllamaAI,
    pub config: GodConfig,
    // The actual live memory of the bot.
    recollections: Vec<ChatMessage>,
}

impl Default for God {
    fn default() -> Self {
        let config = GodConfig::default();
        Self::from_config(config)
    }
}

impl God {
    pub fn get_prompt(&self, author: &str, prompt: &str) -> Vec<ChatMessage> {
        let mut prompts = self.config.thursdayism.clone();
        prompts.append(&mut self.recollections.clone());
        prompts.push(ChatMessage::user(format!("{author}: {prompt}").to_owned()));
        return prompts;
    }

    pub fn set_prompt_response(&mut self, author: &str, prompt: &str, response: &str) {
        self.recollections.push(ChatMessage::user(
            format!("{author}: {}", prompt).to_owned(),
        ));
        self.recollections
            .push(ChatMessage::assistant(response.to_owned()));

        if self.recollections.len() > (MAX_RECOLLECTIONS * 2) {
            self.recollections.remove(0);
            self.recollections.remove(0);
        }
    }

    pub fn set_context(&mut self, context: &str) {
        self.config.thursdayism[0] = ChatMessage::system(context.to_owned());
    }

    pub fn set_botname(&mut self, name: &str) {
        self.config.botname = name.to_string();
    }

    pub fn get_botname(&self) -> String {
        self.config.botname.clone()
    }

    // Remove recollections
    pub fn clear(&mut self) {
        self.recollections.clear();
    }

    // Remove both recollections and thursdayism
    pub fn clear_interactions(&mut self) {
        self.recollections.clear();
        let context = self.config.thursdayism.first().unwrap().to_owned();
        self.config.thursdayism.clear();
        self.config.thursdayism.push(context);
    }

    pub fn add_interaction(&mut self, author: &str, prompt: &str, response: &str) {
        self.config.thursdayism.push(ChatMessage::user(
            format!("{author}: {}", prompt).to_owned(),
        ));
        self.config
            .thursdayism
            .push(ChatMessage::assistant(response.to_owned()));
    }

    pub fn from_config(config: GodConfig) -> God {
        God {
            brain: OllamaAI::new(&config.model, config.options.clone()),
            recollections: Vec::new(),
            config,
        }
    }

    pub fn update_from_config(&mut self, config: GodConfig) {
        self.brain = OllamaAI::new(&config.model, config.options.clone());
        self.recollections = Vec::new();
        self.config = config;
    }

    pub fn export_json(&self) -> serde_json::Value {
        json!(self.config)
    }

    pub fn import_json(val: &str) -> Option<Self> {
        if let Ok(config) = serde_json::from_str::<GodConfig>(val) {
            Some(Self::from_config(config))
        } else {
            None
        }
    }
    pub fn get_config(&self) -> String {
        let memory: String = self
            .config
            .thursdayism
            .iter()
            .map(|x| match x.role {
                MessageRole::System => format!("System: {}\n\n", x.content),
                MessageRole::Assistant => format!("bot: {}\n", x.content),
                MessageRole::User => format!("{}\n", x.content),
            })
            .collect();
        let recollections: String = self
            .recollections
            .iter()
            .map(|x| match x.role {
                MessageRole::System => format!("System: {}\\nn", x.content),
                MessageRole::Assistant => format!("bot: {}\n", x.content),
                MessageRole::User => format!("{}\n", x.content),
            })
            .collect();
        format!(
            "{botname} config.
===========
Initial memory:
---------------
{memory}
Current memory:
---------------
{recollections}\n",
            botname = self.config.botname,
            memory = memory,
            recollections = recollections,
        )
    }
}
