mod config;
mod repository;
mod service;
mod model;
mod graphql;

use crate::graphql::{Mutation, Query, ServerContext, Subscription};
use axum::routing::{get, post, get_service};
use axum::{Extension, Router};
use async_graphql::http::{GraphiQLSource, playground_source, GraphQLPlaygroundConfig};
use async_graphql::Schema as AsyncSchema;
use async_graphql_axum::{GraphQLRequest, GraphQLResponse, GraphQLSubscription};
use std::net::SocketAddr;
use tokio::net::TcpListener;
use tower::ServiceBuilder;
use tower_http::cors::{Any, CorsLayer};
use tower_http::services::ServeDir;
use axum::response::Html;

pub type Schema = AsyncSchema<Query, Mutation, Subscription>;

async fn graphql_handler(
    Extension(schema): Extension<Schema>,
    req: GraphQLRequest,
) -> GraphQLResponse {
    schema.execute(req.into_inner()).await.into()
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
        .allow_headers(Any);

    let server_context = ServerContext::default();
    let schema = Schema::build(Query, Mutation, Subscription {})
        .data(server_context.clone())
        .finish();

    let app = Router::new()
        .route("/graphql", post(graphql_handler).get(graphiql))
        .route("/subscriptions", get_service(GraphQLSubscription::new(schema.clone())))
        .route("/graphiql", get(graphiql))
        .route("/playground", get(playground))
        .layer(Extension(schema))
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