use axum::extract::{Json, Path, State};
use axum::{routing::get, routing::post, Router, Server};
use rand::seq::SliceRandom;
use rand::thread_rng;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::sync::{Arc, Mutex};
use std::{collections::HashMap, str};

// 0 - 9, a - z, A - Z
const POSSIBLE_CHARS: [u8; 62] = [
    48, 49, 50, 51, 52, 53, 54, 55, 56, 57, 65, 66, 67, 68, 69, 70, 71, 72, 73, 74, 75, 76, 77, 78,
    79, 80, 81, 82, 83, 84, 85, 86, 87, 88, 89, 90, 97, 98, 99, 100, 101, 102, 103, 104, 105, 106,
    107, 108, 109, 110, 111, 112, 113, 114, 115, 116, 117, 118, 119, 120, 121, 122,
];

#[derive(Deserialize, Serialize, Debug)]
struct ShortenPayload {
    url: String,
}

#[derive(Clone)]
struct AppState {
    short_urls: Arc<Mutex<HashMap<String, String>>>,
}

#[tokio::main]
async fn main() {
    let state = AppState {
        short_urls: Arc::new(Mutex::new(HashMap::new())),
    };

    let app = Router::new()
        .route("/:unique_id", get(get_url))
        .route("/api/v1/shorten", post(post_shorten))
        .with_state(state);

    let addr = "127.0.0.1:3000".parse().unwrap();
    let server = Server::bind(&addr).serve(app.into_make_service());

    println!("Listening on {addr}...");

    server.await.unwrap();
}

async fn get_url(State(state): State<AppState>, Path(unique_id): Path<String>) -> Json<Value> {
    let short_urls = state.short_urls.lock().unwrap();
    match short_urls.get(&unique_id) {
        Some(url) => Json(json!({ "url": url.clone() })),
        None => Json(json!({ "error": String::from("No URL for this id") })),
    }
}

async fn post_shorten(
    State(state): State<AppState>,
    Json(payload): Json<ShortenPayload>,
) -> Json<Value> {
    let unique_id = random_string(7);
    let short_url = create_short_url(&unique_id);

    let mut short_urls = state.short_urls.lock().unwrap();

    if short_urls.contains_key(&unique_id) {
        Json(json!({ "error": "Could not generate short URL" }))
    } else {
        short_urls.insert(unique_id, payload.url);
        Json(json!({ "short_url": short_url }))
    }
}

fn random_string(size: usize) -> String {
    let mut rng = thread_rng();
    let mut arr: Vec<u8> = vec![0; size];

    for i in 0..size {
        arr[i] = *POSSIBLE_CHARS.choose(&mut rng).unwrap();
    }

    let s = String::from_utf8(arr).unwrap();
    s.to_string()
}

fn create_short_url(unique_id: &str) -> String {
    format!("https://su.at/{}", unique_id)
}
