use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Challenge {
    /// SHA256 hash (recommended)
    S256,
    /// Plain text (less secure, fallback only)
    Plain,
}

impl Challenge {
    pub fn as_str(&self) -> &str {
        match self {
            Challenge::S256 => "S256",
            Challenge::Plain => "plain",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoginParams {
    /// KeyCloak issuer URL (e.g., "http://127.0.0.1:8888/realms/hackandlearn")
    pub issuer: String,

    /// OAuth client ID (e.g., "berlin-rust-client")
    pub client_id: String,

    /// Redirect URI after authentication (e.g., "http://127.0.0.1:8081/")
    pub redirect_uri: String,

    /// OAuth scopes (e.g., "openid profile email")
    pub scope: Option<String>,

    /// Token audience (optional)
    pub audience: Option<String>,

    /// PKCE challenge method
    pub challenge: Challenge,
}

impl LoginParams {
    pub fn new(issuer: String, client_id: String, redirect_uri: String) -> Self {
        Self {
            issuer,
            client_id,
            redirect_uri,
            scope: Some("openid profile email".to_string()),
            audience: None,
            challenge: Challenge::S256,
        }
    }

    pub fn with_scope(mut self, scope: String) -> Self {
        self.scope = Some(scope);
        self
    }

    pub fn with_audience(mut self, audience: String) -> Self {
        self.audience = Some(audience);
        self
    }

    pub fn with_challenge(mut self, challenge: Challenge) -> Self {
        self.challenge = challenge;
        self
    }
}
