use jsonwebtoken::{decode, decode_header, DecodingKey, Validation, Algorithm};
use serde::{Deserialize, Serialize};
use axum::{
    extract::Request,
    middleware::Next,
    response::Response,
    http::{StatusCode, header},
};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RealmAccess {
    pub roles: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClientAccess {
    pub roles: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    pub sub: String,
    pub exp: usize,
    pub iat: usize,
    pub iss: String,
    pub aud: Option<serde_json::Value>,
    pub preferred_username: Option<String>,
    pub email: Option<String>,
    pub realm_access: Option<RealmAccess>,
    pub resource_access: Option<HashMap<String, ClientAccess>>,
}

impl Claims {
    pub fn get_realm_roles(&self) -> Vec<String> {
        self.realm_access
            .as_ref()
            .map(|ra| ra.roles.clone())
            .unwrap_or_default()
    }

    pub fn get_client_roles(&self, client_id: &str) -> Vec<String> {
        self.resource_access
            .as_ref()
            .and_then(|ra| ra.get(client_id))
            .map(|ca| ca.roles.clone())
            .unwrap_or_default()
    }

    pub fn has_role(&self, client_id: &str, role: &str) -> bool {
        self.get_realm_roles().iter().any(|r| r == role)
            || self.get_client_roles(client_id).iter().any(|r| r == role)
    }
}

pub struct AuthConfig {
    pub issuer: String,
    pub jwks_url: String,
    pub audience: String,
}

pub struct AuthState {
    pub config: AuthConfig,
    pub jwks: RwLock<Option<jsonwebtoken::jwk::JwkSet>>,
}

impl AuthState {
    pub fn new(issuer: String, jwks_url: String, audience: String) -> Self {
        Self {
            config: AuthConfig { issuer, jwks_url, audience },
            jwks: RwLock::new(None),
        }
    }

    pub async fn get_jwks(&self) -> Result<jsonwebtoken::jwk::JwkSet, String> {
        {
            let jwks = self.jwks.read().await;
            if let Some(jwks) = jwks.as_ref() {
                return Ok(jwks.clone());
            }
        }

        let jwks = reqwest::get(&self.config.jwks_url)
            .await
            .map_err(|e| e.to_string())?
            .json::<jsonwebtoken::jwk::JwkSet>()
            .await
            .map_err(|e| e.to_string())?;

        let mut jwks_write = self.jwks.write().await;
        *jwks_write = Some(jwks.clone());
        Ok(jwks)
    }
}

pub async fn auth_middleware(
    axum::extract::State(state): axum::extract::State<Arc<AuthState>>,
    mut req: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    let auth_header = req.headers()
        .get(header::AUTHORIZATION)
        .and_then(|h| h.to_str().ok())
        .ok_or(StatusCode::UNAUTHORIZED)?;

    if !auth_header.starts_with("Bearer ") {
        return Err(StatusCode::UNAUTHORIZED);
    }

    let token = &auth_header[7..];

    let header = decode_header(token).map_err(|_| StatusCode::UNAUTHORIZED)?;
    let kid = header.kid.ok_or(StatusCode::UNAUTHORIZED)?;

    let jwks = state.get_jwks().await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let jwk = jwks.find(&kid).ok_or(StatusCode::UNAUTHORIZED)?;

    let decoding_key = DecodingKey::from_jwk(jwk).map_err(|_| StatusCode::UNAUTHORIZED)?;

    let mut validation = Validation::new(Algorithm::RS256);
    validation.set_issuer(&[&state.config.issuer]);
    validation.set_audience(&[&state.config.audience]);

    let token_data = decode::<Claims>(token, &decoding_key, &validation)
        .map_err(|e| {
            tracing::error!("Token validation failed: {:?}", e);
            StatusCode::UNAUTHORIZED
        })?;

    req.extensions_mut().insert(token_data.claims);

    Ok(next.run(req).await)
}