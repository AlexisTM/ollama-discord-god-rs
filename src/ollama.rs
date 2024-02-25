use ollama_rs::{
    generation::{
        chat::{request::ChatMessageRequest, ChatMessage},
        options::GenerationOptions,
    },
    Ollama,
};

#[derive(Debug)]
pub struct OllamaAI {
    ollama: Ollama,
    options: GenerationOptions,
    pub model: String,
}

impl OllamaAI {
    pub fn new(model: &str, options: GenerationOptions) -> Self {
        Self {
            ollama: Ollama::default(),
            options,
            model: model.to_owned(),
        }
    }

    pub async fn request(&self, messages: &[ChatMessage]) -> Option<ChatMessage> {
        let request = ChatMessageRequest::new(self.model.clone(), messages.to_owned());
        let response = self
            .ollama
            .send_chat_messages(request.options(self.options.clone()))
            .await;
        if let Ok(response) = response {
            return response.message;
        }
        None
    }
}
