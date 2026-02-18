use std::collections::{HashMap, HashSet};
use serde::{Deserialize, Serialize};
use thiserror::Error;
use wasm_bindgen::prelude::*;

// ============================================================================
// Error Types
// ============================================================================

#[derive(Error, Debug)]
pub enum AuthError {
    #[error("Network error: {0}")]
    NetworkError(String),

    #[error("JWT validation error: {0}")]
    JwtValidationError(String),

    #[error("Claims extraction error: {0}")]
    ClaimsExtractionError(String),

    #[error("OIDC configuration error: {0}")]
    OidcConfigError(String),

    #[error("PKCE error: {0}")]
    PkceError(String),

    #[error("OAuth error: {0}")]
    OAuthError(String),

    #[error("Storage error: {0}")]
    StorageError(String),

    #[error("Invalid parameter: {0}")]
    InvalidParameter(String),
}

// Convert AuthError to JsValue for WASM
impl From<AuthError> for JsValue {
    fn from(error: AuthError) -> Self {
        JsValue::from_str(&error.to_string())
    }
}

// ============================================================================
// PKCE Challenge Method
// ============================================================================

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

// ============================================================================
// Login Parameters
// ============================================================================

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

// ============================================================================
// OIDC Configuration
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OidcConfig {
    pub issuer: String,
    pub authorization_endpoint: String,
    pub token_endpoint: String,
    pub jwks_uri: String,
    pub userinfo_endpoint: Option<String>,
    pub end_session_endpoint: Option<String>,
}

// ============================================================================
// JWT Claims
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Claims {
    /// Subject - unique user identifier
    pub sub: String,

    /// Issuer - who issued the token
    pub iss: String,

    /// Audience - who the token is intended for
    pub aud: Option<serde_json::Value>,

    /// Expiration time (Unix timestamp)
    pub exp: i64,

    /// Issued at time (Unix timestamp)
    pub iat: i64,

    /// Not before time (Unix timestamp)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nbf: Option<i64>,

    /// JWT ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub jti: Option<String>,

    /// Preferred username
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preferred_username: Option<String>,

    /// Email address
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,

    /// Email verified
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email_verified: Option<bool>,

    /// Given name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub given_name: Option<String>,

    /// Family name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub family_name: Option<String>,

    /// Full name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// KeyCloak realm access roles
    #[serde(skip_serializing_if = "Option::is_none")]
    pub realm_access: Option<RealmAccess>,

    /// KeyCloak resource access roles (client_id -> roles)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_access: Option<HashMap<String, ClientAccess>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RealmAccess {
    pub roles: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClientAccess {
    pub roles: Vec<String>,
}

impl Claims {
    /// Check if the token is expired
    pub fn is_expired(&self) -> bool {
        let now = js_sys::Date::now() / 1000.0;
        self.exp < now as i64
    }

    /// Get user ID (subject)
    pub fn get_user_id(&self) -> &str {
        &self.sub
    }

    /// Get email if available
    pub fn get_email(&self) -> Option<&str> {
        self.email.as_deref()
    }

    /// Get username if available
    pub fn get_username(&self) -> Option<&str> {
        self.preferred_username.as_deref()
    }

    /// Get full name if available
    pub fn get_name(&self) -> Option<&str> {
        self.name.as_deref()
    }

    pub fn get_roles(&self, client_id: &str) -> Vec<String> {
        self.get_realm_roles()
            .into_iter()
            .chain(self.get_client_roles(client_id))
            .collect::<HashSet<_>>() // Entfernt Duplikate
            .into_iter()
            .collect()
    }

    /// Get realm-level roles
    pub fn get_realm_roles(&self) -> Vec<String> {
        self.realm_access
            .as_ref()
            .map(|ra| ra.roles.clone())
            .unwrap_or_default()
    }

    /// Get roles for a specific client from resource_access
    pub fn get_client_roles(&self, client_id: &str) -> Vec<String> {
        self.resource_access
            .as_ref()
            .and_then(|ra| ra.get(client_id))
            .map(|ca| ca.roles.clone())
            .unwrap_or_default()
    }

    /// Check if user has a specific role in realm or ressources
    pub fn has_role(&self, client_id: &str, role: &str) -> bool {
        self.get_roles(client_id).iter().any(|r| r == role)
    }

    /// Check if user has a specific realm role
    pub fn has_realm_role(&self, role: &str) -> bool {
        self.get_realm_roles().iter().any(|r| r == role)
    }

    /// Check if user has a specific role in a given client
    pub fn has_client_role(&self, client_id: &str, role: &str) -> bool {
        self.get_client_roles(client_id).iter().any(|r| r == role)
    }
}

// ============================================================================
// Token Response
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenResponse {
    pub access_token: String,
    pub token_type: String,
    pub expires_in: i64,
    pub refresh_token: Option<String>,
    pub id_token: Option<String>,
    pub scope: Option<String>,
}

// ============================================================================
// PKCE State
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct PkceState {
    pub code_verifier: String,
    pub code_challenge: String,
    pub state: String,
}

// ============================================================================
// Module declarations
// ============================================================================

pub mod oidc;
mod pkce;
mod auth;
pub mod validation;
mod storage;

// Re-export public API
pub use auth::{login, logout, login_and_get_claims, handle_redirect_callback};
pub use validation::extract_claims;