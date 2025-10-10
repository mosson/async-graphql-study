use std::sync::Arc;

use async_graphql::{
    EmptyMutation, EmptySubscription, Object, Response, Schema, Variables,
    http::{GraphQLPlaygroundConfig, playground_source},
};
use axum::{
    http::Method, response::{Html, IntoResponse}, routing::get, Extension, Json, Router
};
use serde::Deserialize;
use sqlx::sqlite::SqlitePoolOptions;
use tower_http::cors::{self, CorsLayer};

#[derive(Deserialize, Debug, Clone)]
#[allow(dead_code)]
struct GraphQlRequest {
    query: String,
    operation_name: Option<String>,
    variables: Option<serde_json::Value>,
}

fn cors() -> CorsLayer {
    CorsLayer::new()
        .allow_headers(cors::Any)
        .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE])
        .allow_origin(cors::Any)
}

async fn playground() -> impl IntoResponse {
    Html(playground_source(GraphQLPlaygroundConfig::new("/")))
}

async fn graphql_handler(
    Extension(schema): Extension<Arc<Schema<Query, EmptyMutation, EmptySubscription>>>,
    Json(payload): Json<GraphQlRequest>,
) -> Json<Response> {
    let mut request = async_graphql::Request::new(payload.query);
    if let Some(operation_name) = payload.operation_name {
        request = request.operation_name(operation_name);
    }
    if let Some(variables) = payload.variables {
        request = request.variables(Variables::from_json(variables));
    }

    schema.execute(request).await.into()
}

struct Query;

#[Object]
impl Query {
    async fn ping(&self) -> String {
        "PONG".into()
    }

    async fn var(&self, id: usize) -> usize {
        id
    }
}

#[tokio::main]
async fn main() {
    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect(":memory:")
        .await
        .unwrap();
    let schema = Schema::build(Query, EmptyMutation, EmptySubscription)
        .data(pool)
        .finish();

    let app = Router::new()
        .route("/", get(playground).post(graphql_handler))
        .layer(cors())
        .layer(Extension(Arc::new(schema)));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3333").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
