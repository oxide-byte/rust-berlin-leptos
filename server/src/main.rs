mod config;
mod repository;
mod service;
mod model;
mod graphql;
mod auth;

use crate::graphql::{Mutation, Query, ServerContext, Subscription};
use crate::auth::{AuthState, auth_middleware};
use axum::routing::{get, post, get_service};
use axum::{Extension, Router};
use async_graphql::http::{GraphiQLSource, playground_source, GraphQLPlaygroundConfig};
use async_graphql::Schema as AsyncSchema;
use async_graphql_axum::{GraphQLRequest, GraphQLResponse, GraphQLSubscription};
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::net::TcpListener;
use tower::ServiceBuilder;
use tower_http::cors::{Any, CorsLayer};
use axum::http::header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE};
use axum::middleware;
use tower_http::services::ServeDir;
use axum::response::Html;

pub type Schema = AsyncSchema<Query, Mutation, Subscription>;

async fn graphql_handler(
    Extension(schema): Extension<Schema>,
    claims: Option<Extension<crate::auth::Claims>>,
    req: GraphQLRequest,
) -> GraphQLResponse {
    let mut request = req.into_inner();
    if let Some(Extension(claims)) = claims {
        request = request.data(claims);
    }
    schema.execute(request).await.into()
}

async fn graphiql() -> Html<String> {
    Html(GraphiQLSource::build().endpoint("/graphql").subscription_endpoint("/subscriptions").finish())
}

async fn playground() -> Html<String> {
    let cfg = GraphQLPlaygroundConfig::new("/graphql").subscription_endpoint("/subscriptions");
    Html(playground_source(cfg))
}

#[tokio::main]
async fn main() {
    // initialize tracing
    tracing_subscriber::fmt::init();

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods([axum::http::Method::GET, axum::http::Method::POST])
        .allow_headers([AUTHORIZATION, CONTENT_TYPE, ACCEPT]);

    let server_context = ServerContext::default();
    let schema = Schema::build(Query, Mutation, Subscription {})
        .data(server_context.clone())
        .finish();

    let auth_state = Arc::new(AuthState::new(
        "http://localhost:8888/realms/hackandlearn".to_string(),
        "http://localhost:8888/realms/hackandlearn/protocol/openid-connect/certs".to_string(),
        "hackandlearn-client".to_string(),
    ));

    // Protect only the HTTP GraphQL endpoint with auth middleware; leave WS subscriptions open
    let graphql_http = Router::new()
        .route("/graphql", post(graphql_handler))
        .layer(middleware::from_fn_with_state(auth_state.clone(), auth_middleware));

    let app = Router::new()
        .merge(graphql_http)
        .route("/subscriptions", get_service(GraphQLSubscription::new(schema.clone())))
        .route("/graphiql", get(graphiql))
        .route("/playground", get(playground))
        .route("/graphql", get(graphiql))
        .nest_service("/public", ServeDir::new("static/public"))
        .nest_service("/web", ServiceBuilder::new().service(ServeDir::new("static")))
        .layer(Extension(schema))
        .layer(cors);

    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    let listener = TcpListener::bind(addr).await
        .unwrap_or_else(|e| panic!("failed to listen on {addr}: {e}"));

    tracing::info!("Server on:        http://{addr}");
    tracing::info!("Leptos client on: http://{addr}/web");
    tracing::info!("graphiql on:      http://{addr}/graphiql");
    tracing::info!("playground on:    http://{addr}/playground");

    axum::serve(listener, app).await
        .unwrap_or_else(|e| panic!("failed to run `axum::serve`: {e}"));
}