use axum::{
    routing::post, 
    routing::get,
    Router
};

use axum::http::Method;

use std::net::SocketAddr;
use tower_http::cors::{Any, CorsLayer};

use crate::redis::worker::start_worker_handler;
use crate::sockets::sockets::ws_handler;
use crate::types::PagesData;
use crate::{
    preview::handlers::receive_preview
};

use tokio::sync::watch;

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

     let initial_data = PagesData { 
        pages : vec![],
        user_id: Option::Some("Hello".to_string()), 
        user_name: Option::Some("Hello".to_string()), 
        site_name: Option::Some("Hello".to_string()), 
        project_id: Option::Some("Hello".to_string()), 
        project_name: Option::Some("Hello".to_string())
    };

    let (_tx, rx) = watch::channel::<PagesData>(initial_data);

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

    // //* START REDIS WORKER HERE
    // tokio::spawn(async move {
    //     start_worker_handler("sclera:jobs", rx).await;
    //     println!("WORKER STARTED...");
    // });

    axum_server::bind(addr)
        .serve(app.into_make_service())
        .await
        .unwrap();

}
