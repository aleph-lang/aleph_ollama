use std::env;
use std::fs;
use reqwest::blocking::Client;
use reqwest::header::{HeaderMap, CONTENT_TYPE};
use serde_json::{json, Value};

pub fn translate_code(file_path: &str, target_language: &str) -> Result<String, String> {
    let code = fs::read_to_string(file_path).map_err(|e| format!("Error reading the file: {}", e))?;
    let model = env::var("OLLAMA_MODEL").unwrap_or_else(|_| "qwen2.5-coder".to_string());

    let client = Client::new();
    let url = env::var("OLLAMA_URL").unwrap_or_else(|_| "http://localhost:11434/api/generate".to_string());
    let body = json!({
        "model": model,
        "prompt": format!(
            "Translate the following code to {}. Output only the code:\n{}", 
            target_language, code
            ),
            "stream": true
    });

    let mut headers = HeaderMap::new();
    headers.insert(CONTENT_TYPE, "application/json".parse().unwrap());

    let response = client
        .post(url)
        .headers(headers)
        .json(&body)
        .send()
        .map_err(|e| format!("Error sending request to Ollama: {}", e))?;

    if !response.status().is_success() {
        return Err(format!("Ollama error: HTTP status {}", response.status()));
    }

    let response_text = response
        .text()
        .map_err(|e| format!("Error retrieving response: {}", e))?;

    let mut result = String::new();
    let mut skip_first_line = true;
    for line in response_text.lines() {
        if let Ok(json_value) = serde_json::from_str::<Value>(line) {
            if let Some(response_content) = json_value.get("response") {
                if let Some(content) = response_content.as_str() {
                    if content.contains("<|im_start|>") || content.contains("```") {
                        continue;
                    }
                    if skip_first_line && !content.trim().is_empty() {
                        skip_first_line = false;
                        continue;
                    }
                    result.push_str(content);
                }
            }
        }
    }

    // Nettoyage final
    let clean_result = result
        .lines()
        .skip_while(|line| line.trim().is_empty() || line.contains("<|im_start|>") || line.contains("```"))
        .take_while(|line| !line.contains("```"))
        .collect::<Vec<&str>>()
        .join("\n");

    Ok(clean_result.trim().to_string())
}
