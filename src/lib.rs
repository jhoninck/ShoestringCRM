pub mod config;
pub mod db;
pub mod model;
pub mod orm;
pub mod graphql;
pub mod security;

use axum::Router;
use graphql::create_schema;
use async_graphql_axum::GraphQL;

pub async fn create_app() -> anyhow::Result<Router> {
    let schema = create_schema();
    let app = Router::new()
        .route("/graphql", axum::routing::post(GraphQL::new(schema)));

    Ok(app)
}
