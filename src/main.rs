use std::sync::Arc;

use axum::{
    Router,
    http::{
        Method,
        header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE},
    },
    routing::{get, post, delete},
};
use tower_http::cors::CorsLayer;

use skrunkly_draw_backend::{State, api, config::CONFIG};

#[tokio::main]
async fn main() {
    env_logger::init();

    let cors_layer = CorsLayer::new()
        .allow_origin(CONFIG.server.cors.allowed_origins.clone())
        .allow_methods([Method::GET, Method::PUT, Method::DELETE])
        .allow_headers([AUTHORIZATION, ACCEPT, CONTENT_TYPE]);

    let router = Router::new()
        .route("/v0/post/all", get(api::post::get_all))
        .route("/v0/post", post(api::post::post))
        .route("/v0/post", delete(api::post::delete))
        .layer(cors_layer)
        .with_state(Arc::new(State::new().await));

    log::info!("Starting server at {}...", CONFIG.server.bind_address);
    let listener = tokio::net::TcpListener::bind(CONFIG.server.bind_address)
        .await
        .unwrap();
    axum::serve(listener, router).await.unwrap();
}
