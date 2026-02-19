use crate::validation::{extract_claims, JwtHeader};

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
