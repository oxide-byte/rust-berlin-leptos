mod error;
mod types;
mod auth;
pub mod oidc;
mod pkce;
pub mod validation;
mod storage;

#[cfg(test)]
mod tests;

pub use error::AuthError;
pub use types::{Challenge, Claims, ClientAccess, LoginParams, OidcConfig, RealmAccess, TokenResponse};
pub(crate) use types::PkceState;

pub use auth::{handle_redirect_callback, login, login_and_get_claims, logout};
pub use validation::extract_claims;
