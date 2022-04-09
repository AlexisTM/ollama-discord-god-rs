use crate::ai21::{Intellect, AI21};
use std::borrow::Borrow;
use std::env;
use std::fmt;

pub struct Discussion(pub Vec<DiscussionKind>);

pub struct AIMemory {
    context: String,
    thursdayism: Discussion,
    recollections: Discussion,
}

pub enum DiscussionKind {
    Prompt { author: String, prompt: String },
    Response { author: String, prompt: String },
}

impl fmt::Display for DiscussionKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DiscussionKind::Prompt { author, prompt } => {
                write!(f, "{}: {}\n", author, prompt)
            }
            DiscussionKind::Response { author, prompt } => {
                write!(f, "{}: {}\n\n", author, prompt)
            }
        }
    }
}

impl fmt::Display for Discussion {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for val in &self.0[0..self.0.len() - 1] {
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
}

// trait Bot, for Kirby
#[derive(Debug)]
pub struct Kirby {
    botname: String,
    pub brain: Box<dyn Intellect + Sync + Send>,
    pub memory: Box<AIMemory>,
}

impl AIMemory {
    pub fn new(context: String, thursdayism: Discussion) -> AIMemory {
        AIMemory {
            context: context,
            thursdayism: thursdayism,
            recollections: Discussion(Vec::new()),
        }
    }

    pub fn get_prompt(&mut self, author: &str, prompt: &str, botname: &str) -> String {
        self.recollections.push(DiscussionKind::Prompt {
            author: author.to_string(),
            prompt: prompt.to_string(),
        });
        return format!("{}\n{}:", self.to_string(), botname);
    }

    pub fn set_response(&mut self, author: &str, prompt: &str) {
        self.recollections.push(DiscussionKind::Response {
            author: author.to_string(),
            prompt: prompt.to_string(),
        });
    }

    pub fn clear(&mut self) {
        self.recollections.clear();
    }

    pub fn to_string(&self) -> String {
        let mut result = String::new();
        result.push_str(&self.context);
        result.push_str("\n\n");

        if self.recollections.len() <= 6 {
            result.push_str(&self.thursdayism.to_string());
        }

        for val in &self.recollections.0[0..self.recollections.len()] {
            result.push_str(&val.to_string());
        }

        return result;
    }
}

impl Kirby {
    pub fn new(botname: &str) -> Kirby {
        let token_ai21 =
            env::var("GOD_AI21_TOKEN").expect("Expected a token in the environment for AI21");
        let initial_prompt: Discussion = Discussion(
            vec![DiscussionKind::Prompt{author: "Alexis".to_string(), prompt: "Who is god?".to_string()}, DiscussionKind::Response{author: "Kirby".to_string(), prompt: "Well, now that you ask, I can tell you. I, Kirby is the great goddess is the god of everybody!".to_string()}],
        );
        let memory = AIMemory::new(String::from("Kirby is the god of all beings. Yet, he is the most lovely god and answers in a very complete manner.\n\n"), initial_prompt);

        Kirby {
            botname: botname.to_string(),
            brain: Box::new(AI21 {
                token: token_ai21,
                stop_sequences: vec!["Kirby: ".to_string(), "\n\n\n".to_string()],
                max_tokens: 250,
                temperature: 0.7,
                top_p: 1.0,
            }),
            memory: Box::new(memory),
        }
    }

    pub fn get_prompt(&mut self, author: &str, prompt: &str) -> String {
        self.memory.get_prompt(author, prompt, &self.botname)
    }

    pub fn set_response(&mut self, prompt: &str) {
        self.memory.set_response(&self.botname, prompt)
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
