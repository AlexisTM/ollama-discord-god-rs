use serde_json::Value;
use unescape::unescape;
use async_trait::async_trait;

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

#[derive(Debug)]
pub enum AI21Errors {
  RequestError(reqwest::Error),
  SerdeError(serde_json::Error),
}
impl std::fmt::Display for AI21Errors {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
      match self {
          AI21Errors::RequestError(parse_int_error) =>
              write!(f, "{}", parse_int_error),
          AI21Errors::SerdeError(io_error) =>
              write!(f, "{}", io_error),
      }
  }
}
impl std::error::Error for AI21Errors {}
impl From<reqwest::Error> for AI21Errors {
  fn from(err: reqwest::Error) -> Self {
      AI21Errors::RequestError(err)
  }
}

impl From<serde_json::Error> for AI21Errors {
  fn from(err: serde_json::Error) -> Self {
      AI21Errors::SerdeError(err)
  }
}

impl AI21 {
  pub async fn request(
    &self,
    token_ai21: &str,
    prompt: &str,
    max_tokens: i32,
    stop_sequences: &Vec<String>,
    temperature: f32,
    top_p: f32,
  ) -> Result<String, AI21Errors> {
    let url = format!(
      "https://api.ai21.com/studio/v1/{model}/complete",
      model = "j1-jumbo"
    );

    let body_obj = serde_json::json!({
        "prompt": prompt.to_string(),
        "num_results_str": "1".to_string(),
        "max_tokens_str": max_tokens.to_string(),
        "stop_sequences": stop_sequences,
        "temperature_str": temperature.to_string(),
        "top_p_str": top_p.to_string(),
        "top_k_return_str": "0".to_string(),
    });

    let bearer = format!("Bearer {}", token_ai21);
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
      .and_then(|value| value.as_str()).unwrap();
    return Ok(ai_completion.to_string());
  }
}

#[async_trait]
impl Intellect for AI21 {
  async fn request(&self, prompt: &str) -> String {
    let result = self.request(
      self.token.as_str(),
      prompt,
      self.max_tokens,
      &self.stop_sequences,
      self.temperature,
      self.top_p,
    );
    match result.await {
      Ok(n) => return unescape(&n).unwrap(),
      Err(_) => return "".to_string(),
    }
  }
}

unsafe impl Sync for AI21 {}
unsafe impl Send for AI21 {}
