use crate::{AuthError, Challenge, PkceState};
use base64::engine::general_purpose::URL_SAFE_NO_PAD;
use base64::Engine;
use getrandom::getrandom;
use sha2::{Digest, Sha256};

/// Generate a cryptographically secure random string for PKCE code verifier
/// Length between 43-128 characters as per RFC 7636
fn generate_random_string(length: usize) -> Result<String, AuthError> {
    if length < 43 || length > 128 {
        return Err(AuthError::PkceError(
            "Code verifier length must be between 43 and 128 characters".to_string(),
        ));
    }

    let mut bytes = vec![0u8; length];
    getrandom(&mut bytes).map_err(|e| AuthError::PkceError(format!("Random generation failed: {}", e)))?;

    // Convert to base64 URL-safe encoding (no padding)
    let verifier = URL_SAFE_NO_PAD.encode(&bytes);

    // Truncate to exact length
    Ok(verifier.chars().take(length).collect())
}

/// Generate code challenge from code verifier using specified method
fn generate_code_challenge(verifier: &str, method: &Challenge) -> String {
    match method {
        Challenge::S256 => {
            // SHA256 hash
            let mut hasher = Sha256::new();
            hasher.update(verifier.as_bytes());
            let hash = hasher.finalize();

            // Base64 URL-safe encoding (no padding)
            URL_SAFE_NO_PAD.encode(hash)
        }
        Challenge::Plain => {
            // Plain text (not recommended, but supported)
            verifier.to_string()
        }
    }
}

/// Generate a random state parameter for CSRF protection
fn generate_state() -> Result<String, AuthError> {
    let mut bytes = vec![0u8; 32];
    getrandom(&mut bytes).map_err(|e| AuthError::PkceError(format!("State generation failed: {}", e)))?;

    Ok(URL_SAFE_NO_PAD.encode(&bytes))
}

/// Generate complete PKCE state (verifier, challenge, and state parameter)
pub(crate) fn generate_pkce_state(challenge_method: &Challenge) -> Result<PkceState, AuthError> {
    // Generate code verifier (128 chars for maximum entropy)
    let code_verifier = generate_random_string(128)?;

    // Generate code challenge from verifier
    let code_challenge = generate_code_challenge(&code_verifier, challenge_method);

    // Generate random state for CSRF protection
    let state = generate_state()?;

    Ok(PkceState {
        code_verifier,
        code_challenge,
        state,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_random_string() {
        let result = generate_random_string(43);
        assert!(result.is_ok());
        let s = result.unwrap();
        assert_eq!(s.len(), 43);
    }

    #[test]
    fn test_generate_random_string_invalid_length() {
        assert!(generate_random_string(42).is_err());
        assert!(generate_random_string(129).is_err());
    }

    #[test]
    fn test_generate_code_challenge_s256() {
        let verifier = "dBjftJeZ4CVP-mB92K27uhbUJU1p1r_wW1gFWFOEjXk";
        let challenge = generate_code_challenge(verifier, &Challenge::S256);

        // Expected value from RFC 7636 example
        // Note: This is the standardized test vector
        assert_eq!(challenge, "E9Melhoa2OwvFrEMTJguCHaoeK1t8URWbuGJSstw-cM");
    }

    #[test]
    fn test_generate_code_challenge_plain() {
        let verifier = "test_verifier";
        let challenge = generate_code_challenge(verifier, &Challenge::Plain);
        assert_eq!(challenge, verifier);
    }

    #[test]
    fn test_generate_state() {
        let result = generate_state();
        assert!(result.is_ok());
        let state = result.unwrap();
        assert!(!state.is_empty());
    }

    #[test]
    fn test_generate_pkce_state() {
        let result = generate_pkce_state(&Challenge::S256);
        assert!(result.is_ok());

        let pkce = result.unwrap();
        assert_eq!(pkce.code_verifier.len(), 128);
        assert!(!pkce.code_challenge.is_empty());
        assert!(!pkce.state.is_empty());
    }
}
