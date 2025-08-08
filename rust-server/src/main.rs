use axum::{
    routing::post, 
    routing::get,
    Router
};

use axum::http::Method;

use std::net::SocketAddr;
use tower_http::cors::{Any, CorsLayer};

use crate::sockets::sockets::ws_handler;
use crate::{
    preview::handlers::receive_preview
};

mod handlers;
mod models;
mod renders;
mod react;
mod types;
mod preview;
mod utils;
mod constants;
mod render_helpers;
mod sockets;
mod redis;
mod runners;

#[tokio::main]
async fn main() {

    println!("🔥 entered main");

    println!("✅ after redis_connection_handler");

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(vec![Method::POST, Method::GET])
        .allow_headers(vec![axum::http::header::CONTENT_TYPE]);

    let app = Router::new()
        .route("/ws", get(ws_handler))
        .route("/preview", post(handlers::preview))
        .route("/react-preview", post(react::file_writer::react_preview))
        .route("/preview-request", post(receive_preview))
        .layer(cors);

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    println!("Server running on http://{}", addr);

    axum_server::bind(addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
