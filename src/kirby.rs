use crate::ai21::{Intellect, AI21};
use std::env;
use std::fmt;

// By using a tuple, I can implement Display for Vec<DiscussionKind>
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

#[derive(Clone)]
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

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
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
}

impl Kirby {
    pub fn new(botname: &str) -> Kirby {
        let token_ai21 =
            env::var("GOD_AI21_TOKEN").expect("Expected a token in the environment for AI21");

        let initial_prompt: Discussion = Discussion(
            vec![
                DiscussionKind::Prompt{
                    author: "Alexis".to_string(),
                    prompt: "Who is god?".to_string()
                },
                DiscussionKind::Response{
                    author: "Kirby".to_string(),
                    prompt: "Well, now that you ask, I can tell you. I, Kirby is the great goddess is the god of everybody!".to_string()
                }],
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

    pub fn get_prompt(&self, author: &str, prompt: &str) -> String {
        self.memory.get_prompt(author, prompt, &self.botname)
    }

    pub fn set_prompt_response(&mut self, author: &str, prompt: &str, response: &str) {
        self.memory.set_prompt(author, prompt);
        self.memory.set_response(&self.botname, response)
    }

    pub fn clear(&mut self) {
        self.memory.clear();
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
