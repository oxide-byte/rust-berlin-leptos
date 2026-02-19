use std::collections::{HashMap, HashSet};

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RealmAccess {
    pub roles: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClientAccess {
    pub roles: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Claims {
    /// Subject - unique user identifier
    pub sub: String,

    /// Issuer - who issued the token
    pub iss: String,

    /// Audience - who the token is intended for
    pub aud: Option<serde_json::Value>,

    /// Expiration time (Unix timestamp)
    pub exp: i64,

    /// Issued at time (Unix timestamp)
    pub iat: i64,

    /// Not before time (Unix timestamp)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nbf: Option<i64>,

    /// JWT ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub jti: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub preferred_username: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub email_verified: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub given_name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub family_name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// KeyCloak realm-level roles
    #[serde(skip_serializing_if = "Option::is_none")]
    pub realm_access: Option<RealmAccess>,

    /// KeyCloak resource-level roles (client_id -> roles)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_access: Option<HashMap<String, ClientAccess>>,
}

impl Claims {
    pub fn is_expired(&self) -> bool {
        let now = js_sys::Date::now() / 1000.0;
        self.exp < now as i64
    }

    pub fn get_user_id(&self) -> &str {
        &self.sub
    }

    pub fn get_email(&self) -> Option<&str> {
        self.email.as_deref()
    }

    pub fn get_username(&self) -> Option<&str> {
        self.preferred_username.as_deref()
    }

    pub fn get_name(&self) -> Option<&str> {
        self.name.as_deref()
    }

    /// Get combined realm + client roles (deduped)
    pub fn get_roles(&self, client_id: &str) -> Vec<String> {
        self.get_realm_roles()
            .into_iter()
            .chain(self.get_client_roles(client_id))
            .collect::<HashSet<_>>()
            .into_iter()
            .collect()
    }

    pub fn get_realm_roles(&self) -> Vec<String> {
        self.realm_access
            .as_ref()
            .map(|ra| ra.roles.clone())
            .unwrap_or_default()
    }

    pub fn get_client_roles(&self, client_id: &str) -> Vec<String> {
        self.resource_access
            .as_ref()
            .and_then(|ra| ra.get(client_id))
            .map(|ca| ca.roles.clone())
            .unwrap_or_default()
    }

    pub fn has_role(&self, client_id: &str, role: &str) -> bool {
        self.get_roles(client_id).iter().any(|r| r == role)
    }

    pub fn has_realm_role(&self, role: &str) -> bool {
        self.get_realm_roles().iter().any(|r| r == role)
    }

    pub fn has_client_role(&self, client_id: &str, role: &str) -> bool {
        self.get_client_roles(client_id).iter().any(|r| r == role)
    }
}
