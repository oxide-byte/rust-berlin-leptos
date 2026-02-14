use crate::{
    storage::{retrieve_pkce_state, store_pkce_state, store_token},
    validation::validate_and_extract_claims,
    AuthError, Claims, LoginParams, TokenResponse,
};
use url::Url;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;
use web_sys::window;

/// Build the OAuth 2.0 authorization URL with PKCE parameters
fn build_authorization_url(
    auth_endpoint: &str,
    params: &LoginParams,
    code_challenge: &str,
    code_challenge_method: &str,
    state: &str,
) -> Result<String, AuthError> {
    let mut url = Url::parse(auth_endpoint)
        .map_err(|e| AuthError::InvalidParameter(format!("Invalid authorization endpoint: {}", e)))?;

    url.query_pairs_mut()
        .append_pair("client_id", &params.client_id)
        .append_pair("redirect_uri", &params.redirect_uri)
        .append_pair("response_type", "code")
        .append_pair("code_challenge", code_challenge)
        .append_pair("code_challenge_method", code_challenge_method)
        .append_pair("state", state);

    if let Some(scope) = &params.scope {
        url.query_pairs_mut().append_pair("scope", scope);
    }

    Ok(url.to_string())
}

/// Initiate the OAuth login flow
/// This will redirect the browser to the KeyCloak login page
pub async fn login(params: LoginParams) -> Result<(), AuthError> {
    web_sys::console::log_1(&"[KeyCloak Auth] login() function called".into());
    web_sys::console::log_1(&format!("[KeyCloak Auth] Issuer: {}", params.issuer).into());

    // Fetch OIDC configuration
    let oidc_config = crate::oidc::fetch_oidc_config(&params.issuer).await?;
    web_sys::console::log_1(&"[KeyCloak Auth] ✅ OIDC config fetched".into());

    // Generate PKCE state
    let pkce_state = crate::pkce::generate_pkce_state(&params.challenge)?;
    web_sys::console::log_1(&format!("[KeyCloak Auth] ✅ PKCE state generated (challenge: {} chars)", pkce_state.code_challenge.len()).into());

    // Store PKCE state for later retrieval in callback
    web_sys::console::log_1(&"[KeyCloak Auth] About to store PKCE state...".into());
    store_pkce_state(&pkce_state)?;
    web_sys::console::log_1(&"[KeyCloak Auth] ✅ PKCE state stored successfully".into());

    // Build authorization URL
    let auth_url = build_authorization_url(
        &oidc_config.authorization_endpoint,
        &params,
        &pkce_state.code_challenge,
        params.challenge.as_str(),
        &pkce_state.state,
    )?;
    web_sys::console::log_1(&format!("[KeyCloak Auth] ✅ Authorization URL built: {}", auth_url).into());

    // Redirect to authorization URL
    let window = window().ok_or_else(|| AuthError::OAuthError("No window object".to_string()))?;
    let location = window.location();

    web_sys::console::log_1(&"[KeyCloak Auth] 🚀 REDIRECTING TO KEYCLOAK NOW...".into());

    location
        .set_href(&auth_url)
        .map_err(|e| AuthError::OAuthError(format!("Failed to redirect: {:?}", e)))?;

    web_sys::console::log_1(&"[KeyCloak Auth] This line should never be reached".into());

    Ok(())
}

/// Handle the OAuth redirect callback
/// Call this function when the user is redirected back to your app after login
pub async fn handle_redirect_callback(params: LoginParams) -> Result<String, AuthError> {
    let window = window().ok_or_else(|| AuthError::OAuthError("No window object".to_string()))?;
    let location = window.location();

    // Get the current URL with query parameters
    let href = location
        .href()
        .map_err(|e| AuthError::OAuthError(format!("Failed to get URL: {:?}", e)))?;

    let url = Url::parse(&href)
        .map_err(|e| AuthError::InvalidParameter(format!("Invalid redirect URL: {}", e)))?;

    // Extract authorization code and state from query parameters
    let mut code: Option<String> = None;
    let mut state: Option<String> = None;
    let mut error: Option<String> = None;
    let mut error_description: Option<String> = None;

    for (key, value) in url.query_pairs() {
        match key.as_ref() {
            "code" => code = Some(value.to_string()),
            "state" => state = Some(value.to_string()),
            "error" => error = Some(value.to_string()),
            "error_description" => error_description = Some(value.to_string()),
            _ => {}
        }
    }

    // Check for OAuth errors
    if let Some(err) = error {
        let description = error_description.unwrap_or_else(|| "Unknown error".to_string());
        return Err(AuthError::OAuthError(format!("{}: {}", err, description)));
    }

    // Validate we have the required parameters
    let code = code.ok_or_else(|| AuthError::OAuthError("No authorization code in callback".to_string()))?;

    let state = state.ok_or_else(|| AuthError::OAuthError("No state parameter in callback".to_string()))?;

    // Retrieve and validate PKCE state
    let pkce_state = retrieve_pkce_state()?;

    if pkce_state.state != state {
        return Err(AuthError::OAuthError(
            "State parameter mismatch (CSRF protection)".to_string(),
        ));
    }

    // Fetch OIDC configuration
    let oidc_config = crate::oidc::fetch_oidc_config(&params.issuer).await?;

    // Exchange authorization code for tokens
    let token_response = exchange_code_for_token(
        &oidc_config.token_endpoint,
        &code,
        &pkce_state.code_verifier,
        &params,
    )
    .await?;

    // Store access token
    store_token(&token_response.access_token)?;

    // Return the access token (or id_token if you prefer)
    Ok(token_response.access_token)
}

/// Exchange authorization code for access token
async fn exchange_code_for_token(
    token_endpoint: &str,
    code: &str,
    code_verifier: &str,
    params: &LoginParams,
) -> Result<TokenResponse, AuthError> {
    let window = window().ok_or_else(|| AuthError::NetworkError("No window object".to_string()))?;

    // Build URL-encoded form body (required by OAuth2 spec)
    let body = format!(
        "grant_type=authorization_code&code={}&client_id={}&redirect_uri={}&code_verifier={}",
        urlencoding::encode(code),
        urlencoding::encode(&params.client_id),
        urlencoding::encode(&params.redirect_uri),
        urlencoding::encode(code_verifier)
    );

    web_sys::console::log_1(&format!("[KeyCloak Auth] Token request body: {}", body).into());

    // Create request
    let opts = web_sys::RequestInit::new();
    opts.set_method("POST");
    opts.set_mode(web_sys::RequestMode::Cors);
    opts.set_body(&wasm_bindgen::JsValue::from_str(&body));

    let request = web_sys::Request::new_with_str_and_init(token_endpoint, &opts)
        .map_err(|e| AuthError::NetworkError(format!("Failed to create request: {:?}", e)))?;

    // Set Content-Type header for URL-encoded form data
    request
        .headers()
        .set("Content-Type", "application/x-www-form-urlencoded")
        .map_err(|e| AuthError::NetworkError(format!("Failed to set Content-Type header: {:?}", e)))?;

    // Make request
    let resp_value = JsFuture::from(window.fetch_with_request(&request))
        .await
        .map_err(|e| AuthError::NetworkError(format!("Token request failed: {:?}", e)))?;

    let resp: web_sys::Response = resp_value
        .dyn_into()
        .map_err(|_| AuthError::NetworkError("Invalid response type".to_string()))?;

    if !resp.ok() {
        // Try to get the error response body
        let error_text = if let Ok(json_promise) = resp.text() {
            if let Ok(text_value) = JsFuture::from(json_promise).await {
                if let Some(text) = text_value.as_string() {
                    web_sys::console::log_1(&format!("[KeyCloak Auth] Token endpoint error response: {}", text).into());
                    text
                } else {
                    "Could not parse error response".to_string()
                }
            } else {
                "Could not read error response".to_string()
            }
        } else {
            "No error response body".to_string()
        };

        return Err(AuthError::OAuthError(format!(
            "Token exchange failed: HTTP {} - {} | Response: {}",
            resp.status(),
            resp.status_text(),
            error_text
        )));
    }

    // Parse response
    let json = JsFuture::from(
        resp.json()
            .map_err(|e| AuthError::NetworkError(format!("Failed to parse JSON: {:?}", e)))?,
    )
    .await
    .map_err(|e| AuthError::NetworkError(format!("Failed to get JSON: {:?}", e)))?;

    let token_response: TokenResponse = serde_wasm_bindgen::from_value(json)
        .map_err(|e| AuthError::OAuthError(format!("Failed to deserialize token response: {:?}", e)))?;

    Ok(token_response)
}

/// Complete login flow and return validated claims
/// This is a convenience function that combines login initiation and callback handling
pub async fn login_and_get_claims(params: LoginParams) -> Result<Claims, AuthError> {
    // Check if we're in a callback (has 'code' parameter)
    let window = window().ok_or_else(|| AuthError::OAuthError("No window object".to_string()))?;
    let location = window.location();

    let href = location
        .href()
        .map_err(|e| AuthError::OAuthError(format!("Failed to get URL: {:?}", e)))?;

    let url = Url::parse(&href)
        .map_err(|e| AuthError::InvalidParameter(format!("Invalid URL: {}", e)))?;

    let has_code = url
        .query_pairs()
        .any(|(key, _)| key == "code");

    if has_code {
        // We're in the callback, handle it
        let token = handle_redirect_callback(params.clone()).await?;

        // Fetch OIDC config for JWKS URI
        let oidc_config = crate::oidc::fetch_oidc_config(&params.issuer).await?;

        // Validate token and extract claims
        let claims = validate_and_extract_claims(
            &token,
            &oidc_config.jwks_uri,
            &params.issuer,
            params.audience.as_deref(),
        )
        .await?;

        Ok(claims)
    } else {
        // Initiate login (will redirect, so this won't return)
        login(params).await?;

        // This line will never be reached due to redirect,
        // but we need it for type checking
        Err(AuthError::OAuthError("Redirect initiated".to_string()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Challenge;

    #[test]
    fn test_build_authorization_url() {
        let params = LoginParams {
            issuer: "http://localhost:8888/realms/test".to_string(),
            client_id: "test-client".to_string(),
            redirect_uri: "http://localhost:8081/".to_string(),
            scope: Some("openid profile email".to_string()),
            audience: None,
            challenge: Challenge::S256,
        };

        let url = build_authorization_url(
            "http://localhost:8888/auth",
            &params,
            "test-challenge",
            "S256",
            "test-state",
        );

        assert!(url.is_ok());
        let url_str = url.unwrap();

        assert!(url_str.contains("client_id=test-client"));
        assert!(url_str.contains("redirect_uri=http%3A%2F%2Flocalhost%3A8081%2F"));
        assert!(url_str.contains("response_type=code"));
        assert!(url_str.contains("code_challenge=test-challenge"));
        assert!(url_str.contains("code_challenge_method=S256"));
        assert!(url_str.contains("state=test-state"));
        assert!(url_str.contains("scope=openid+profile+email"));
    }
}
