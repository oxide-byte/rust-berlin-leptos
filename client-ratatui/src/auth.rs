use reqwest::Client;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenResponse {
    pub access_token: String,
    pub expires_in: u64,
    pub refresh_expires_in: u64,
    pub refresh_token: String,
    pub token_type: String,
}

const KEYCLOAK_TOKEN_URL: &str = "http://localhost:8888/realms/hackandlearn/protocol/openid-connect/token";

pub async fn authenticate(username: &str, password: &str) -> Result<String, String> {
    let client = Client::new();

    let params = [
        ("client_id", "hackandlearn-client"),
        ("username", username),
        ("password", password),
        ("grant_type", "password"),
    ];

    let response = client
        .post(KEYCLOAK_TOKEN_URL)
        .form(&params)
        .send()
        .await
        .map_err(|e| format!("Failed to send request: {}", e))?;

    if !response.status().is_success() {
        return Err(format!("Authentication failed: {}", response.status()));
    }

    let token_response: TokenResponse = response
        .json()
        .await
        .map_err(|e| format!("Failed to parse response: {}", e))?;

    Ok(token_response.access_token)
}