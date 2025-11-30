mod config;
mod repository;
mod service;
mod model;
mod graphql;

use crate::graphql::{Mutation, Query, ServerContext, Subscription};
use axum::routing::{get, on, MethodFilter};
use axum::{Extension, Router};
use juniper::RootNode;
use juniper_axum::extract::JuniperRequest;
use juniper_axum::response::JuniperResponse;
use juniper_axum::{graphiql, playground, ws};
use juniper_graphql_ws::ConnectionConfig;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::net::TcpListener;
use tower::ServiceBuilder;
use tower_http::cors::{Any, CorsLayer};
use tower_http::services::ServeDir;

pub type Schema = RootNode<Query, Mutation, Subscription>;

async fn custom_graphql(
    Extension(schema): Extension<Arc<Schema>>,
    Extension(server_context): Extension<ServerContext>,
    JuniperRequest(request): JuniperRequest,
) -> JuniperResponse {
    JuniperResponse(request.execute(&*schema, &server_context).await)
}

#[tokio::main]
async fn main() {
    // initialize tracing
    tracing_subscriber::fmt::init();

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods([axum::http::Method::GET, axum::http::Method::POST])
        .allow_headers(Any);

    let schema = Schema::new(Query, Mutation, Subscription {});

    let server_context = ServerContext::default();

    let app = Router::new()
        .route("/graphql", on(MethodFilter::GET.or(MethodFilter::POST), custom_graphql))
        .route("/subscriptions", get(ws::<Arc<Schema>>(ConnectionConfig::new(server_context.clone()))))
        .route("/graphiql", get(graphiql("/graphql", "/subscriptions")))
        .route("/playground", get(playground("/graphql", "/subscriptions")))
        .layer(Extension(Arc::new(schema)))
        .layer(Extension(server_context))
        .layer(cors)
        .nest_service(
            "/web",
            ServiceBuilder::new()
                .service(ServeDir::new("static")),
        );

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