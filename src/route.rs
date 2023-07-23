mod channel_select;

use axum::http::Request;
use axum::middleware::Next;
use axum::{extract::Query, response::IntoResponse, routing::get, Extension, Json, Router};
use lambda_http::Body;
use serde::Deserialize;
use serde_json::json;
use std::sync::Arc;
use tracing::info;

use crate::prisma::{channel, PrismaClient, SortOrder};
use channel_select::{channel_select_weight, channel_select_without_id};

type Client = Extension<Arc<PrismaClient>>;

#[derive(Deserialize)]
struct ChannelFilter {
    weight: Option<String>,
    order: Option<String>,
}

async fn get_channel(client: Client, querys: Option<Query<ChannelFilter>>) -> impl IntoResponse {
    let mut finds: Vec<i32> = vec![];
    let mut order: SortOrder = SortOrder::Asc;

    if let Some(q) = querys {
        if let Some(w) = q.weight.clone() {
            let fl = w
                .split(',')
                .map(|n| n.replace(" ", "").parse::<i32>().unwrap_or(-1));
            for f in fl {
                if f < 0 {
                    continue;
                }
                finds.push(f);
            }
        }
        if let Some(o) = q.order.clone() {
            if o.to_lowercase().as_str() == "desc" {
                order = SortOrder::Desc;
            }
        }
    }

    #[allow(bindings_with_variant_name)]
    let channels = client
        .channel()
        .find_many(if finds.len() == 0 {
            vec![]
        } else {
            vec![channel::weight::in_vec(finds)]
        })
        .order_by(channel::weight::order(order))
        .select(channel_select_without_id::select())
        .exec()
        .await
        .unwrap();

    Json(json!({ "channels": channels }))
}

async fn get_channel_count(client: Client) -> impl IntoResponse {
    // NOTE: I known we can just do channels len but just to show how count work as a note for myself
    let count = client.channel().count(vec![]).exec().await.unwrap();
    let channels: Vec<i32> = client
        .channel()
        .find_many(vec![])
        .order_by(channel::weight::order(SortOrder::Asc))
        .select(channel_select_weight::select())
        .exec()
        .await
        .unwrap()
        .iter()
        .map(|c| c.weight)
        .collect();

    Json(json!({ "count": count, "channels": channels }))
}

async fn get_name() -> impl IntoResponse {
    use fake::{faker::name::raw::*, locales::*, Fake};

    let name: String = FirstName(EN).fake();
    let lastname: String = LastName(EN).fake();
    Json(json!({ "name": name, "lastname": lastname }))
}

pub fn get_fake_route() -> Router<(), Body> {
    Router::new().route("/name", get(get_name))
}

pub async fn info_middleware<B>(request: Request<B>, next: Next<B>) -> impl IntoResponse {
    let uri = request.uri().path_and_query().unwrap().to_string();
    info!("{uri}");

    next.run(request).await
}

pub fn get_channel_route(client: Arc<PrismaClient>) -> Router<(), Body> {
    Router::new()
        .route("/get", get(get_channel))
        .route("/count", get(get_channel_count))
        .layer(Extension(client))
}
