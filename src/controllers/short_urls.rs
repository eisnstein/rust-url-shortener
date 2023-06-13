use axum::extract::{Json, Path, State};
use serde_json::{json, Value};
use std::sync::Arc;

use crate::utilities;
use crate::AppState;
use crate::Data;
use crate::ShortenPayload;

pub async fn get_url(
    State(state): State<Arc<AppState>>,
    Path(unique_id): Path<String>,
) -> Json<Value> {
    let short_urls = state.short_urls.lock().unwrap();
    match short_urls.get(&unique_id) {
        Some(data) => Json(json!({ "url": data.surl.clone() })),
        None => Json(json!({ "error": String::from("No URL for this id") })),
    }
}

pub async fn post_shorten(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<ShortenPayload>,
) -> Json<Value> {
    let unique_id = utilities::random_string(7);
    let short_url = utilities::create_short_url(&unique_id);

    let mut short_urls = state.short_urls.lock().unwrap();

    if short_urls.contains_key(&unique_id) {
        Json(json!({ "error": "Could not generate short URL" }))
    } else {
        short_urls.insert(
            unique_id,
            Data {
                surl: payload.url,
                is_synced: false,
            },
        );
        Json(json!({ "short_url": short_url }))
    }
}
