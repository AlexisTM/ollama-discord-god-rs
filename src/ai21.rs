use crate::error::GodError;
use async_trait::async_trait;
use serde_json::Value;
use unescape::unescape;

#[async_trait]
pub trait Intellect {
    async fn request(&self, prompt: &str) -> String;
}

pub struct AI21 {
    pub token: String,
    pub stop_sequences: Vec<String>,
    pub max_tokens: i32,
    pub temperature: f32,
    pub top_p: f32,
}

impl AI21 {
    pub async fn request(
        &self,
        prompt: &str,
    ) -> Result<String, GodError> {
        let url = format!(
            "https://api.ai21.com/studio/v1/{model}/complete",
            model = "j1-jumbo"
        );

        let body_obj = serde_json::json!({
            "prompt": prompt.to_string(),
            "numResults": 1,
            "maxTokens": self.max_tokens,
            "stopSequences": self.stop_sequences,
            "temperature": self.temperature,
            "topP": self.top_p,
            "topKReturn": 0,
        });

        let bearer = format!("Bearer {}", self.token);
        let client = reqwest::Client::new();
        let response = client
            .post(url)
            .header(reqwest::header::AUTHORIZATION, &bearer)
            .header(reqwest::header::ACCEPT, "application/json")
            .header(reqwest::header::CONTENT_TYPE, "application/json")
            //.body(body_obj.to_string())
            .json(&body_obj)
            .send()
            .await?
            .text()
            .await?;

        let response_json: Value = serde_json::from_str(&response)?;

        let ai_completion = response_json
            .get("completions")
            .and_then(|value| value.get(0))
            .and_then(|value| value.get("data"))
            .and_then(|value| value.get("text"))
            .and_then(|value| value.as_str())
            .unwrap();
        Ok(ai_completion.to_string())
    }
}

#[async_trait]
impl Intellect for AI21 {
    async fn request(&self, prompt: &str) -> String {
        let result = self.request(
            prompt,
        );
        match result.await {
            Ok(n) => return unescape(&n).unwrap(),
            Err(_) => return "".to_string(),
        }
    }
}
