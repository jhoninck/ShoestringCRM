/*
HTTP client for communicating with the local or remote Ollama runtime.

Responsibilities:
- send prompt requests
- deserialize responses
- centralize model/runtime configuration
- handle transport-level errors
*/
use serde::{Deserialize, Serialize};

#[derive(Clone)]
pub struct OllamaClient {
    base_url: String,
}

impl OllamaClient {
    pub fn new(base_url: &str) -> Self {
        Self {
            base_url: base_url.to_string(),
        }
    }

    pub async fn generate(&self, prompt: &str) -> Result<String, String> {

        let client = reqwest::Client::new();

        let req = OllamaRequest {
            model: "llama3".to_string(),
            prompt: prompt.to_string(),
            stream: false,
        };

        let res = client
            .post(format!("{}/api/generate", self.base_url))
            .json(&req)
            .send()
            .await
            .map_err(|e| e.to_string())?;

        let body: OllamaResponse = res.json().await.map_err(|e| e.to_string())?;

        Ok(body.response)
    }
}

#[derive(Serialize)]
struct OllamaRequest {
    model: String,
    prompt: String,
    stream: bool,
}

#[derive(Deserialize)]
struct OllamaResponse {
    response: String,
}
