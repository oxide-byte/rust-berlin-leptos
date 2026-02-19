use crate::pkce::{generate_code_challenge, generate_pkce_state, generate_random_string, generate_state};
use crate::Challenge;

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
