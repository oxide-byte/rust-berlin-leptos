use leptos::logging::log;
use leptos::prelude::*;
use reactive_stores::{Patch, Store};

#[derive(Clone, Debug, Default, Store, Patch)]
pub struct GlobalState {
    pub token: Option<String>,
    pub user_id: Option<String>,
    pub email: Option<String>,
    pub username: Option<String>,
    pub name: Option<String>,
    pub roles: Vec<String>,
    pub is_authenticated: bool,
}

#[component]
pub fn KeyCloakCatcher() -> impl IntoView {
    let state = expect_context::<Store<GlobalState>>();

    // Check for OAuth callback on page load - run only once on mount
    let state_clone = state.clone();
    leptos::task::spawn_local(async move {
            use keycloak_wasm_auth::{handle_redirect_callback, Challenge};

            // Test sessionStorage availability
            if let Some(window) = leptos::web_sys::window() {
                if let Ok(Some(storage)) = window.session_storage() {
                    // Try to write a test value
                    match storage.set_item("_test_key", "test_value") {
                        Ok(_) => {
                            log!("[KeyCloak] ✅ SessionStorage is working (write test passed)");
                            let _ = storage.remove_item("_test_key");
                        }
                        Err(e) => {
                            log!("[KeyCloak] ❌ SessionStorage write FAILED: {:?}", e);
                            log!("[KeyCloak] ⚠️  This may indicate browser privacy settings blocking storage");
                        }
                    }
                } else {
                    log!("[KeyCloak] ❌ SessionStorage is NOT available!");
                }
            }

            // Check if already authenticated - skip callback processing
            if state_clone.is_authenticated().get() {
                log!("[KeyCloak] Already authenticated, skipping callback");
                return;
            }

            // Check if we're being redirected back from KeyCloak
            if let Some(window) = leptos::web_sys::window() {
                if let Ok(href) = window.location().href() {
                    log!("[KeyCloak] Current URL: {}", href);

                    if href.contains("code=") && href.contains("state=") {
                        log!("[KeyCloak] Detected OAuth callback in URL");

                        // Debug: Check what's in sessionStorage
                        if let Ok(Some(storage)) = window.session_storage() {
                            log!("[KeyCloak] SessionStorage length: {}", storage.length().unwrap_or(0));

                            // Try to read the PKCE state key directly (correct key name)
                            match storage.get_item("keycloak_pkce_state") {
                                Ok(Some(val)) => log!("[KeyCloak] ✅ Found keycloak_pkce_state: {} chars", val.len()),
                                Ok(None) => log!("[KeyCloak] ❌ keycloak_pkce_state NOT FOUND in sessionStorage"),
                                Err(e) => log!("[KeyCloak] ❌ Error reading keycloak_pkce_state: {:?}", e),
                            }

                            // List all keys in sessionStorage for debugging
                            if let Ok(length) = storage.length() {
                                log!("[KeyCloak] Listing all sessionStorage keys:");
                                for i in 0..length {
                                    if let Ok(Some(key)) = storage.key(i) {
                                        log!("[KeyCloak]   Key {}: {}", i, key);
                                    }
                                }
                            }
                        } else {
                            log!("[KeyCloak] ❌ Could not access sessionStorage");
                        }

                        let params = keycloak_wasm_auth::LoginParams::new(
                            crate::auth_config::KEYCLOAK_ISSUER.to_string(),
                            crate::auth_config::KEYCLOAK_CLIENT_ID.to_string(),
                            crate::auth_config::KEYCLOAK_REDIRECT_URI.to_string(),
                        )
                        .with_scope(crate::auth_config::KEYCLOAK_SCOPE.to_string())
                        .with_challenge(Challenge::S256);

                        match handle_redirect_callback(params.clone()).await {
                            Ok(token) => {
                                log!("[KeyCloak] ✅ Callback handled successfully");
                                log!("[KeyCloak] Token: {}", token);

                                // Fetch OIDC config for validation
                                if let Ok(oidc_config) = keycloak_wasm_auth::oidc::fetch_oidc_config(&params.issuer).await {
                                    if let Ok(claims) = keycloak_wasm_auth::validation::validate_and_extract_claims(
                                        &token,
                                        &oidc_config.jwks_uri,
                                        &params.issuer,
                                        params.audience.as_deref(),
                                    ).await {
                                        let user_id = claims.get_user_id().to_string();
                                        let email = claims.get_email().map(String::from);
                                        let username = claims.get_username().map(String::from);
                                        let name = claims.get_name().map(String::from);
                                        let roles = claims.get_roles(&params.client_id);

                                        log!("[KeyCloak] User ID: {}", user_id);
                                        log!("[KeyCloak] Email: {}", email.as_deref().unwrap_or("N/A"));
                                        log!("[KeyCloak] Username: {}", username.as_deref().unwrap_or("N/A"));
                                        log!("[KeyCloak] Name: {}", name.as_deref().unwrap_or("N/A"));
                                        log!("[KeyCloak] Roles: {:?}", roles);

                                        state_clone.token().patch(Some(token));
                                        state_clone.user_id().patch(Some(user_id));
                                        state_clone.email().patch(email);
                                        state_clone.username().patch(username);
                                        state_clone.name().patch(name);
                                        state_clone.roles().patch(roles);
                                        state_clone.is_authenticated().patch(true);

                                        log!("[KeyCloak] State updated, cleaning URL...");

                                        // Clean up URL by removing query parameters
                                        let _ = window.history().and_then(|h| {
                                            h.replace_state_with_url(&wasm_bindgen::JsValue::NULL, "", Some("/"))
                                        });
                                    }
                                }
                            }
                            Err(e) => {
                                log!("[KeyCloak] ❌ Callback error: {}", e);
                            }
                        }
                    }
                }
            }
    });

    view! {}
}