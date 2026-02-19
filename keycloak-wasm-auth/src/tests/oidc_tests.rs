// Note: These tests require a WASM environment.
// Run with: wasm-pack test --headless --chrome

#[cfg(target_arch = "wasm32")]
use wasm_bindgen_test::*;

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen_test]
async fn test_fetch_oidc_config_invalid_url() {
    use crate::oidc::fetch_oidc_config;
    let result = fetch_oidc_config("http://invalid-url-that-does-not-exist").await;
    assert!(result.is_err());
}
