#[allow(warnings, unused)]
mod prisma;

use axum::{
    response::{IntoResponse, Json},
    routing::get,
    Extension, Router,
};
use lambda_http::{run, Error};
use prisma::{channel, PrismaClient, SortOrder};
use serde_json::json;
use std::sync::Arc;
use tower::Layer;
use tower_http::normalize_path::NormalizePathLayer;

type Client = Extension<Arc<PrismaClient>>;

async fn get_channel(client: Client) -> impl IntoResponse {
    let channels = client
        .channel()
        .find_many(vec![])
        .order_by(channel::weight::order(SortOrder::Asc))
        .exec()
        .await
        .unwrap();

    Json(json!({ "channels": channels }))
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    // required to enable CloudWatch error logging by the runtime
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        // disable printing the name of the module in every log line.
        .with_target(false)
        // this needs to be set to false, otherwise ANSI color codes will
        // show up in a confusing manner in CloudWatch logs.
        .with_ansi(false)
        // disabling time is handy because CloudWatch will add the ingestion time.
        .without_time()
        .init();

    let client = Arc::new(
        PrismaClient::_builder()
            .build()
            .await
            .expect("Failed to build prisma client."),
    );

    let router = Router::new()
        .route("/get", get(get_channel))
        .layer(Extension(client));

    let nested_router = Router::new()
        // just found a trick
        // don't create new api from lambda, create api with $stage then manual route to lambda
        .nest("/channel", router);

    let app = NormalizePathLayer::trim_trailing_slash().layer(nested_router);

    run(app).await
}
