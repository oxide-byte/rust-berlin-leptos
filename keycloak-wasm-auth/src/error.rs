use thiserror::Error;
use wasm_bindgen::prelude::*;

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

impl From<AuthError> for JsValue {
    fn from(error: AuthError) -> Self {
        JsValue::from_str(&error.to_string())
    }
}