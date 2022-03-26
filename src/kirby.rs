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
pub struct Kirby {
    pub token: String,
    pub memory: AIMemory,
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
            base_text : base_text,
            base_prompt : base_prompt,
            remembrances : Vec::new(),
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
