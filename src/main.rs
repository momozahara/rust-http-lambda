use axum::{
    extract::{OriginalUri, Path},
    http::StatusCode,
    response::{IntoResponse, Json},
    routing::{get, post},
    Router,
};
use lambda_http::{run, Error};
use serde_json::{json, Value};
use tower::Layer;
use tower_http::normalize_path::NormalizePathLayer;

async fn root() -> Json<Value> {
    Json(json!({ "msg": "I am GET /" }))
}

async fn get_foo() -> Json<Value> {
    Json(json!({ "msg": "I am GET /foo" }))
}

async fn post_foo() -> Json<Value> {
    Json(json!({ "msg": "I am POST /foo" }))
}

async fn post_foo_name(Path(name): Path<String>) -> Json<Value> {
    Json(json!({
        "msg": format!("I am POST /foo/:name, name={name}")
    }))
}

// life saver
async fn notfound_handler(uri: OriginalUri) -> impl IntoResponse {
    (
        StatusCode::NOT_FOUND,
        Json(json!({ "message": "Not Found", "uri": uri.to_string() })),
    )
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

    let router = Router::new()
        .route("/", get(root))
        .route("/foo", get(get_foo).post(post_foo))
        .route("/foo/:name", post(post_foo_name));

    let nested_router = Router::new()
        // I'm not sure this is API Gateway black magic or done by lambda-http crate
        // https://id.execute-api.region.amazonaws.com/default/fn_name <-- what you think it is
        // https://id.execute-api.region.amazonaws.com/default/default/fn_name <-- what it actually is
        .nest("/:stage/:stage/:fnname", router)
        .fallback(notfound_handler);

    let app = NormalizePathLayer::trim_trailing_slash().layer(nested_router);

    run(app).await
}
