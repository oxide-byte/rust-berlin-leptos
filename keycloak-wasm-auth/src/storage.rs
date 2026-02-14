use crate::{AuthError, PkceState};
use web_sys::window;

const PKCE_STATE_KEY: &str = "keycloak_pkce_state";

/// Store PKCE state in browser's sessionStorage
pub(crate) fn store_pkce_state(state: &PkceState) -> Result<(), AuthError> {
    let window = window().ok_or_else(|| AuthError::StorageError("No window object".to_string()))?;

    let storage = window
        .session_storage()
        .map_err(|_| AuthError::StorageError("Failed to access sessionStorage".to_string()))?
        .ok_or_else(|| AuthError::StorageError("sessionStorage not available".to_string()))?;

    let state_json = serde_json::to_string(state)
        .map_err(|e| AuthError::StorageError(format!("Failed to serialize state: {}", e)))?;

    storage
        .set_item(PKCE_STATE_KEY, &state_json)
        .map_err(|_| AuthError::StorageError("Failed to store PKCE state".to_string()))?;

    web_sys::console::log_1(&format!("[KeyCloak Storage] ✅ Stored PKCE state with key: {}", PKCE_STATE_KEY).into());
    web_sys::console::log_1(&format!("[KeyCloak Storage] State value: {} chars", state_json.len()).into());

    Ok(())
}

/// Retrieve and remove PKCE state from browser's sessionStorage
pub(crate) fn retrieve_pkce_state() -> Result<PkceState, AuthError> {
    let window = window().ok_or_else(|| AuthError::StorageError("No window object".to_string()))?;

    let storage = window
        .session_storage()
        .map_err(|_| AuthError::StorageError("Failed to access sessionStorage".to_string()))?
        .ok_or_else(|| AuthError::StorageError("sessionStorage not available".to_string()))?;

    web_sys::console::log_1(&format!("[KeyCloak Storage] Attempting to retrieve PKCE state with key: {}", PKCE_STATE_KEY).into());

    // Check what's in sessionStorage
    if let Ok(length) = storage.length() {
        web_sys::console::log_1(&format!("[KeyCloak Storage] SessionStorage has {} items", length).into());

        // List all keys
        for i in 0..length {
            if let Ok(Some(key)) = storage.key(i) {
                web_sys::console::log_1(&format!("[KeyCloak Storage] Key {}: {}", i, key).into());
            }
        }
    }

    let state_json = storage
        .get_item(PKCE_STATE_KEY)
        .map_err(|_| {
            web_sys::console::log_1(&"[KeyCloak Storage] ❌ Failed to call get_item".into());
            AuthError::StorageError("Failed to retrieve PKCE state".to_string())
        })?
        .ok_or_else(|| {
            web_sys::console::log_1(&format!("[KeyCloak Storage] ❌ No PKCE state found with key: {}", PKCE_STATE_KEY).into());
            AuthError::StorageError("No PKCE state found".to_string())
        })?;

    web_sys::console::log_1(&format!("[KeyCloak Storage] ✅ Found PKCE state: {} chars", state_json.len()).into());

    // Remove the state after retrieving (one-time use)
    storage
        .remove_item(PKCE_STATE_KEY)
        .map_err(|_| {
            web_sys::console::log_1(&"[KeyCloak Storage] ⚠️  Failed to remove PKCE state".into());
            AuthError::StorageError("Failed to remove PKCE state".to_string())
        })?;

    web_sys::console::log_1(&"[KeyCloak Storage] ✅ Removed PKCE state from sessionStorage".into());

    let state: PkceState = serde_json::from_str(&state_json)
        .map_err(|e| AuthError::StorageError(format!("Failed to deserialize state: {}", e)))?;

    Ok(state)
}

/// Store token in browser's sessionStorage
pub fn store_token(token: &str) -> Result<(), AuthError> {
    let window = window().ok_or_else(|| AuthError::StorageError("No window object".to_string()))?;

    let storage = window
        .session_storage()
        .map_err(|_| AuthError::StorageError("Failed to access sessionStorage".to_string()))?
        .ok_or_else(|| AuthError::StorageError("sessionStorage not available".to_string()))?;

    storage
        .set_item("keycloak_access_token", token)
        .map_err(|_| AuthError::StorageError("Failed to store token".to_string()))?;

    Ok(())
}

/// Retrieve token from browser's sessionStorage
pub fn retrieve_token() -> Result<String, AuthError> {
    let window = window().ok_or_else(|| AuthError::StorageError("No window object".to_string()))?;

    let storage = window
        .session_storage()
        .map_err(|_| AuthError::StorageError("Failed to access sessionStorage".to_string()))?
        .ok_or_else(|| AuthError::StorageError("sessionStorage not available".to_string()))?;

    storage
        .get_item("keycloak_access_token")
        .map_err(|_| AuthError::StorageError("Failed to retrieve token".to_string()))?
        .ok_or_else(|| AuthError::StorageError("No token found".to_string()))
}

/// Clear all stored auth data
pub fn clear_auth_data() -> Result<(), AuthError> {
    let window = window().ok_or_else(|| AuthError::StorageError("No window object".to_string()))?;

    let storage = window
        .session_storage()
        .map_err(|_| AuthError::StorageError("Failed to access sessionStorage".to_string()))?
        .ok_or_else(|| AuthError::StorageError("sessionStorage not available".to_string()))?;

    storage
        .remove_item("keycloak_access_token")
        .map_err(|_| AuthError::StorageError("Failed to clear token".to_string()))?;

    storage
        .remove_item(PKCE_STATE_KEY)
        .map_err(|_| AuthError::StorageError("Failed to clear PKCE state".to_string()))?;

    Ok(())
}
