use crate::{AuthError, Claims};
use base64::engine::general_purpose::URL_SAFE_NO_PAD;
use base64::Engine;
use serde::Deserialize;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;

/// JWKS (JSON Web Key Set) response structure
#[derive(Debug, Deserialize)]
struct JwksResponse {
    keys: Vec<JwkKey>,
}

#[derive(Debug, Deserialize)]
struct JwkKey {
    kty: String,
    #[serde(rename = "use")]
    key_use: Option<String>,
    kid: Option<String>,
    n: Option<String>,
    e: Option<String>,
    x: Option<String>,
    y: Option<String>,
    crv: Option<String>,
}

/// Fetch JWKS from the KeyCloak server
async fn fetch_jwks(jwks_uri: &str) -> Result<Vec<JwkKey>, AuthError> {
    let window = web_sys::window().ok_or_else(|| AuthError::NetworkError("No window object".to_string()))?;

    let opts = web_sys::RequestInit::new();
    opts.set_method("GET");
    opts.set_mode(web_sys::RequestMode::Cors);

    let request = web_sys::Request::new_with_str_and_init(jwks_uri, &opts)
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
        return Err(AuthError::JwtValidationError(format!(
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

    let jwks: JwksResponse = serde_wasm_bindgen::from_value(json)
        .map_err(|e| AuthError::JwtValidationError(format!("Failed to deserialize JWKS: {:?}", e)))?;

    Ok(jwks.keys)
}

#[derive(Debug, Deserialize)]
struct JwtHeader {
    alg: String,
    kid: Option<String>,
    typ: Option<String>,
}

/// Extract claims from token WITHOUT full cryptographic validation
/// Note: For production use, you should implement full JWT signature verification
/// This simplified version performs basic validation checks
pub fn extract_claims(token: &str) -> Result<Claims, AuthError> {
    let token_parts: Vec<&str> = token.split('.').collect();
    if token_parts.len() != 3 {
        return Err(AuthError::ClaimsExtractionError("Invalid JWT format".to_string()));
    }

    // Decode payload
    let payload_bytes = URL_SAFE_NO_PAD
        .decode(token_parts[1])
        .or_else(|_| {
            // Try with padding if no-padding fails
            base64::engine::general_purpose::STANDARD.decode(token_parts[1])
        })
        .map_err(|e| AuthError::ClaimsExtractionError(format!("Failed to decode payload: {}", e)))?;

    let claims: Claims = serde_json::from_slice(&payload_bytes)
        .map_err(|e| AuthError::ClaimsExtractionError(format!("Failed to parse claims: {}", e)))?;

    Ok(claims)
}

/// Validate JWT token and extract claims
///
/// This function performs:
/// 1. Token structure validation
/// 2. Claims extraction
/// 3. Issuer validation
/// 4. Audience validation (if provided)
/// 5. Expiration check
/// 6. Not-before check (if present)
///
/// Note: Full cryptographic signature verification requires additional implementation
/// For a production system, consider using a more complete JWT validation library
pub async fn validate_and_extract_claims(
    token: &str,
    _jwks_uri: &str, // Reserved for future signature validation
    expected_issuer: &str,
    expected_audience: Option<&str>,
) -> Result<Claims, AuthError> {
    // Extract claims from token
    let claims = extract_claims(token)?;

    // Validate issuer
    if claims.iss != expected_issuer {
        return Err(AuthError::JwtValidationError(format!(
            "Invalid issuer. Expected: {}, Got: {}",
            expected_issuer, claims.iss
        )));
    }

    // Validate audience if provided
    if let Some(expected_aud) = expected_audience {
        let aud_match = match &claims.aud {
            Some(serde_json::Value::String(s)) => s == expected_aud,
            Some(serde_json::Value::Array(arr)) => arr.iter().any(|v| {
                if let serde_json::Value::String(s) = v {
                    s == expected_aud
                } else {
                    false
                }
            }),
            _ => false,
        };

        if !aud_match {
            return Err(AuthError::JwtValidationError(format!(
                "Invalid audience. Expected: {}",
                expected_aud
            )));
        }
    }

    // Validate expiration
    if claims.is_expired() {
        return Err(AuthError::JwtValidationError("Token has expired".to_string()));
    }

    // Validate not before (if present)
    if let Some(nbf) = claims.nbf {
        let now = js_sys::Date::now() / 1000.0;
        if nbf > now as i64 {
            return Err(AuthError::JwtValidationError("Token not yet valid".to_string()));
        }
    }

    Ok(claims)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_claims_invalid_format() {
        let result = extract_claims("invalid.token");
        assert!(result.is_err());
    }

    #[test]
    fn test_jwt_header_parse() {
        let header = r#"{"alg":"RS256","kid":"test-key","typ":"JWT"}"#;
        let parsed: Result<JwtHeader, _> = serde_json::from_str(header);
        assert!(parsed.is_ok());

        let h = parsed.unwrap();
        assert_eq!(h.alg, "RS256");
        assert_eq!(h.kid, Some("test-key".to_string()));
        assert_eq!(h.typ, Some("JWT".to_string()));
    }
}
