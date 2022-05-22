use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::ai21::{Intellect, AI21};
use std::env;
use std::fmt;

// By using a tuple, I can implement Display for Vec<DiscussionKind>
#[derive(Serialize, Deserialize, Clone)]
pub struct Discussion(pub Vec<DiscussionKind>);

// Context is the "Always there"
pub struct AIMemory {
    // Always there, on top of the AI prompt, such as: "This is the discussion between xxx and yyy."
    context: String,
    // We were created last thursday, this is the discussion the bot is born with.
    thursdayism: Discussion,
    // The actual live memory of the bot.
    recollections: Discussion,
}

#[derive(Serialize, Deserialize, Clone)]
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
                writeln!(f, "{}: {}\n", author, prompt)
            }
        }
    }
}

impl fmt::Display for Discussion {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for val in &self.0[0..self.0.len()] {
            write!(f, "{}", val)?
        }
        Ok(())
    }
}

impl Discussion {
    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn clear(&mut self) {
        self.0.clear()
    }

    pub fn push(&mut self, prompt: DiscussionKind) {
        self.0.push(prompt)
    }

    pub fn init(&mut self) {
        self.0 = Vec::new();
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

// trait Bot, for God
#[derive(Debug)]
pub struct God {
    botname: String,
    pub brain: Box<dyn Intellect + Sync + Send>,
    pub memory: Box<AIMemory>,
}

impl AIMemory {
    pub fn new(context: String, thursdayism: Discussion) -> AIMemory {
        AIMemory {
            context,
            thursdayism,
            recollections: Discussion(Vec::new()),
        }
    }

    pub fn get_prompt(&self, author: &str, prompt: &str, botname: &str) -> String {
        let prompt = DiscussionKind::Prompt {
            author: author.to_string(),
            prompt: prompt.to_string(),
        };
        return format!("{}{}\n{}:", self.to_string(), prompt.to_string(), botname);
    }

    pub fn set_prompt(&mut self, author: &str, prompt: &str) {
        self.recollections.push(DiscussionKind::Prompt {
            author: author.to_string(),
            prompt: prompt.to_string(),
        });
    }

    pub fn set_response(&mut self, author: &str, prompt: &str) {
        self.recollections.push(DiscussionKind::Response {
            author: author.to_string(),
            prompt: prompt.to_string(),
        });
        self.clean();
    }

    pub fn clear(&mut self) {
        self.recollections.clear();
    }

    pub fn clean(&mut self) {
        if self.recollections.len() > 12 {
            self.recollections.0 = self.recollections.0
                [self.recollections.len() - 12..self.recollections.len()]
                .to_vec();
        }
    }

    pub fn to_string(&self) -> String {
        if self.recollections.len() <= 6 {
            return format!(
                "{}\n\n{}{}",
                self.context, self.thursdayism, self.recollections
            );
        } else {
            return format!("{}\n\n{}", self.context, self.recollections);
        }
    }

    pub fn clear_interactions(&mut self) {
        self.thursdayism.clear();
    }

    pub fn add_interaction(&mut self, author: &str, prompt: &str, botname: &str, response: &str) {
        self.thursdayism.push(DiscussionKind::Prompt {
            author: author.to_string(),
            prompt: prompt.to_string(),
        });
        self.thursdayism.push(DiscussionKind::Response {
            author: botname.to_string(),
            prompt: response.to_string(),
        });
    }
}

#[derive(Serialize, Deserialize)]
struct GodMemoryConfig {
    pub botname: String,
    pub context: String,
    pub thursdayism: Discussion,
}

impl God {
    pub fn new(botname: &str) -> God {
        let token_ai21 =
            env::var("GOD_AI21_TOKEN").expect("Expected a token in the environment for AI21");

        let initial_prompt: Discussion = Discussion(
            vec![
                DiscussionKind::Prompt{
                    author: "Alexis".to_string(),
                    prompt: "Who is god?".to_string()
                },
                DiscussionKind::Response{
                    author: "God".to_string(),
                    prompt: "Well, now that you ask, I can tell you. I, God is the great goddess is the god of everybody!".to_string()
                }],
        );
        let memory = AIMemory::new(String::from("God is the god of all beings. Yet, he is the most lovely god and answers in a very complete manner.\n\n"), initial_prompt);

        God {
            botname: botname.to_string(),
            brain: Box::new(AI21 {
                token: token_ai21,
                stop_sequences: vec!["God:".to_string(), "\n\n".to_string()],
                max_tokens: 250,
                temperature: 0.7,
                top_p: 1.0,
            }),
            memory: Box::new(memory),
        }
    }

    pub fn get_prompt(&self, author: &str, prompt: &str) -> String {
        self.memory.get_prompt(author, prompt, &self.botname)
    }

    pub fn set_prompt_response(&mut self, author: &str, prompt: &str, response: &str) {
        self.memory.set_prompt(author, prompt);
        self.memory.set_response(&self.botname, response)
    }

    pub fn set_context(&mut self, context: &str) {
        self.memory.context = context.to_string();
    }

    pub fn set_botname(&mut self, name: &str) {
        self.botname = name.to_string();
    }

    pub fn get_botname(&self) -> String {
        self.botname.clone()
    }

    pub fn clear(&mut self) {
        self.memory.clear();
    }

    pub fn clear_interactions(&mut self) {
        self.memory.clear_interactions();
    }

    pub fn add_interaction(&mut self, author: &str, prompt: &str, response: &str) {
        self.memory
            .add_interaction(author, prompt, &self.botname, response);
    }

    pub fn export_json(&self) -> serde_json::Value {
        let config = GodMemoryConfig {
            botname: self.botname.clone(),
            context: self.memory.context.clone(),
            thursdayism: self.memory.thursdayism.clone(),
        };
        json!(config)
    }

    pub fn from_str(val: &str) -> Self {
        let config: GodMemoryConfig = serde_json::from_str(val).unwrap();
        let mut this = Self::new(&config.botname.as_str());
        this.memory.thursdayism = config.thursdayism;
        this.memory.context = config.context;
        this
    }

    pub fn get_config(&self) -> String {
        format!(
            "{botname} config.
===========
Context:
--------
{context}
Initial memory:
---------------
{memory}
Current memory:
---------------
{current_memory}\n",
            botname = self.botname,
            context = self.memory.context,
            memory = self.memory.thursdayism,
            current_memory = self.memory.to_string()
        )
    }
}

impl std::fmt::Debug for Box<AIMemory> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Ohoh, that is a Box<AIMemory!>")
    }
}

impl std::fmt::Debug for Box<dyn Intellect + Sync + Send> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Ohoh, that is a Box<AIMemory!>")
    }
}
