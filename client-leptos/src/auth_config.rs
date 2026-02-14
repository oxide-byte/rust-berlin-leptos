/// KeyCloak authentication configuration
///
/// ⚠️  IMPORTANT: Use EITHER localhost OR 127.0.0.1 consistently!
/// Do NOT mix them - browsers treat them as different origins.
///
/// Current configuration: localhost (recommended for development)
///
/// To switch to 127.0.0.1, change ALL occurrences below AND update:
/// - Your browser URL bar
/// - Trunk serve binding (if configured)
/// - Backend CORS configuration
pub const KEYCLOAK_ISSUER: &str = "http://localhost:8888/realms/hackandlearn";
pub const KEYCLOAK_CLIENT_ID: &str = "hackandlearn-client";
pub const KEYCLOAK_REDIRECT_URI: &str = "http://localhost:8081/";
pub const KEYCLOAK_SCOPE: &str = "openid profile email";

// Backend GraphQL endpoints (must match your backend configuration)
pub const GRAPHQL_HTTP_ENDPOINT: &str = "http://localhost:8080/graphql";
pub const GRAPHQL_WS_ENDPOINT: &str = "ws://localhost:8080/subscriptions";
