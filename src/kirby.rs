use crate::ai21::{Intellect, AI21};
use std::env;
use std::fmt;

pub struct AIPromptResponse {
    pub prompt: String,
    pub response: String,
    pub author: String,
    pub botname: String,
}

pub struct AIMemory {
    base_text: String,
    base_prompt: AIPromptResponse,
    remembrances: Vec<AIPromptResponse>,
}

// trait Bot, for Kirby
#[derive(Debug)]
pub struct Kirby {
    pub brain: Box<dyn Intellect + Sync + Send>,
    pub memory: Box<AIMemory>,
}

impl AIPromptResponse {
    pub fn to_string(&self) -> String {
        return format!(
            "\n{author}: {prompt}\n{botname}: {response}\n",
            author = self.author,
            prompt = self.prompt,
            botname = self.botname,
            response = self.response
        );
    }
}

impl AIMemory {
    pub fn new(base_text: String, base_prompt: AIPromptResponse) -> AIMemory {
        AIMemory {
            base_text: base_text,
            base_prompt: base_prompt,
            remembrances: Vec::new(),
        }
    }

    pub fn update(&mut self, prompt: String, response: String, author: String, botname: String) {
        self.remembrances.push(AIPromptResponse {
            prompt: prompt,
            response: response,
            author: author,
            botname: botname,
        });
    }

    pub fn clear(&mut self) {
        self.remembrances.clear();
    }

    pub fn to_string(&self) -> String {
        let mut result = String::new();
        result.push_str(&self.base_text);
        result.push_str("\n\n");

        if self.remembrances.len() <= 2 {
            result.push_str(&self.base_prompt.to_string());
        }

        for val in &self.remembrances[0..self.remembrances.len()] {
            result.push_str(&val.to_string());
        }

        return result;
    }
}

impl Kirby {
    pub fn new() -> Kirby {
        let token_ai21 =
            env::var("GOD_AI21_TOKEN").expect("Expected a token in the environment for AI21");
        let initial_prompt = AIPromptResponse {
            prompt: "Who is god?".to_string(),
            response: "Well, now that you ask, I can tell you. I, Kirby is the great goddess is the god of everybody!\n".to_string(),
            author: "Alexis".to_string(),
            botname: "Kirby".to_string(),
        };
        let memory = AIMemory::new(String::from("Kirby is the god of all beings. Yet, he is the most lovely god and answers in a very complete manner.\n\n"), initial_prompt);
        Kirby {
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
