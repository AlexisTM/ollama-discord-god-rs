use serde_json::Value;

pub async fn request(
  token_ai21: &str,
  prompt: &str,
  max_tokens: Option<i32>,
  stop_sequences: &str,
  temperature: Option<f32>,
  top_p: Option<f32>,
  top_k_return: Option<i32>,
  num_results: Option<i32>,
) -> Result<String, Box<dyn std::error::Error + Sync + Send>> {
  let url = format!(
    "https://api.ai21.com/studio/v1/{model}/complete",
    model = "j1-jumbo"
  );

  let body_obj = serde_json::json!({
      "prompt": prompt.to_string(),
      "num_results_str": num_results.unwrap_or(1),
      "max_tokens_str": max_tokens.unwrap_or(250),
      "stop_sequences": stop_sequences.to_string(),
      "temperature_str": temperature.unwrap_or(1.0),
      "top_p_str": top_p.unwrap_or(1.0),
      "top_k_return_str": top_k_return.unwrap_or(2),
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
    .unwrap()
    .get(0)
    .unwrap()
    .get("data")
    .unwrap()
    .get("text")
    .unwrap();
  return Ok(ai_completion.to_string());
}
