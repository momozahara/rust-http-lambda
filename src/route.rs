use std::sync::Arc;

use axum::{extract::Query, response::IntoResponse, routing::get, Extension, Json, Router};
use lambda_http::Body;
use serde::Deserialize;
use serde_json::json;

use crate::prisma::{channel, PrismaClient, SortOrder};

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
            let fl = w.split(',').map(|n| n.parse::<i32>().unwrap_or(-1));
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

    let channels = client
        .channel()
        .find_many(vec![channel::weight::in_vec(finds)])
        .order_by(channel::weight::order(order))
        .exec()
        .await
        .unwrap();

    Json(json!({ "channels": channels }))
}

pub fn get_router(client: Arc<PrismaClient>) -> Router<(), Body> {
    let route = Router::new()
        .route("/get", get(get_channel))
        .layer(Extension(client));
    route
}
