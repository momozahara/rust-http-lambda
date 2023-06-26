#[allow(warnings, unused)]
mod prisma;

mod route;

use axum::Router;
use lambda_http::{run, Error};
use prisma::PrismaClient;
use std::sync::Arc;
use tower::Layer;
use tower_http::normalize_path::NormalizePathLayer;

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

    let nested_router = Router::new()
        // channel
        .nest("/channel", route::get_router(client));

    let app = NormalizePathLayer::trim_trailing_slash().layer(nested_router);

    run(app).await
}
