use crate::auth::build_authorization_url;
use crate::{Challenge, LoginParams};

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
