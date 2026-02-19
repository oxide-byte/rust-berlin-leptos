use crate::{AuthError, OidcConfig};
use serde::Deserialize;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;

/// Fetch OIDC configuration from the .well-known endpoint
pub async fn fetch_oidc_config(issuer: &str) -> Result<OidcConfig, AuthError> {
    let config_url = format!("{}/.well-known/openid-configuration", issuer.trim_end_matches('/'));

    // Use web_sys fetch API for WASM compatibility
    let window = web_sys::window().ok_or_else(|| AuthError::NetworkError("No window object".to_string()))?;

    let opts = web_sys::RequestInit::new();
    opts.set_method("GET");
    opts.set_mode(web_sys::RequestMode::Cors);

    let request = web_sys::Request::new_with_str_and_init(&config_url, &opts)
        .map_err(|e| AuthError::NetworkError(format!("Failed to create request: {:?}", e)))?;

    request
        .headers()
        .set("Accept", "application/json")
        .map_err(|e| AuthError::NetworkError(format!("Failed to set headers: {:?}", e)))?;

    let resp_value = JsFuture::from(window.fetch_with_request(&request))
        .await
        .map_err(|e| AuthError::NetworkError(format!("Fetch failed: {:?}", e)))?;

    let resp: web_sys::Response = resp_value
        .dyn_into()
        .map_err(|_| AuthError::NetworkError("Invalid response type".to_string()))?;

    if !resp.ok() {
        return Err(AuthError::OidcConfigError(format!(
            "HTTP error {}: {}",
            resp.status(),
            resp.status_text()
        )));
    }

    let json = JsFuture::from(
        resp.json()
            .map_err(|e| AuthError::NetworkError(format!("Failed to parse JSON: {:?}", e)))?,
    )
    .await
    .map_err(|e| AuthError::NetworkError(format!("Failed to get JSON: {:?}", e)))?;

    let config: OidcConfigResponse = serde_wasm_bindgen::from_value(json)
        .map_err(|e| AuthError::OidcConfigError(format!("Failed to deserialize config: {:?}", e)))?;

    Ok(OidcConfig {
        issuer: config.issuer,
        authorization_endpoint: config.authorization_endpoint,
        token_endpoint: config.token_endpoint,
        jwks_uri: config.jwks_uri,
        userinfo_endpoint: config.userinfo_endpoint,
        end_session_endpoint: config.end_session_endpoint,
    })
}

#[derive(Debug, Deserialize)]
struct OidcConfigResponse {
    issuer: String,
    authorization_endpoint: String,
    token_endpoint: String,
    jwks_uri: String,
    userinfo_endpoint: Option<String>,
    end_session_endpoint: Option<String>,
}
