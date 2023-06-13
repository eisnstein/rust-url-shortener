use axum::{routing::get, routing::post, Router, Server};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};
use std::{collections::HashMap, str};

mod controllers;
mod db;
mod utilities;

#[derive(Deserialize, Serialize, Debug)]
pub struct ShortenPayload {
    url: String,
}

pub struct Data {
    surl: String,
    is_synced: bool,
}

#[derive(Clone)]
pub struct AppState {
    short_urls: Arc<Mutex<HashMap<String, Data>>>,
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt().init();

    tracing::info!("Loading database...");
    let urls = db::load_urls();

    let state = Arc::new(AppState {
        short_urls: Arc::new(Mutex::new(urls)),
    });
    let shared_state = state.clone();

    let app = Router::new()
        .route("/:unique_id", get(controllers::short_urls::get_url))
        .route(
            "/api/v1/shorten",
            post(controllers::short_urls::post_shorten),
        )
        .with_state(state);

    tokio::task::spawn_blocking(move || loop {
        {
            let mut urls = shared_state.short_urls.lock().unwrap();
            db::store_urls(&mut urls);
        }

        std::thread::sleep(std::time::Duration::from_secs(5));
    });

    let addr = "127.0.0.1:3000".parse().unwrap();
    let server = Server::bind(&addr).serve(app.into_make_service());

    tracing::info!("Listening on {addr}...");

    server.await.unwrap();
}
