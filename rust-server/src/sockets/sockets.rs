use axum::extract::ws::{ WebSocket, WebSocketUpgrade };
use axum::response::IntoResponse;
use tokio::sync::watch;
use futures_util::{StreamExt};
use crate::redis::worker::start_worker_handler;
use crate::sockets::socket_event_handlers::socket_receiver_loop_handler;
use crate::types::PagesData;

// * Your WebSocket wrapper handler
pub async fn ws_handler(ws: WebSocketUpgrade) -> impl IntoResponse {
    ws.on_upgrade(handle_socket)
}

pub async fn handle_socket(socket: WebSocket) {
    println!("🔌 WebSocket connected");

    let (sender, receiver) = socket.split();

    let initial_data = PagesData { 
        pages : vec![],
        user_id: Option::Some("Hello".to_string()), 
        user_name: Option::Some("Hello".to_string()), 
        site_name: Option::Some("Hello".to_string()), 
        project_id: Option::Some("Hello".to_string()), 
        project_name: Option::Some("Hello".to_string())
    };

    // * Create a watch channel
    let (tx, rx) = watch::channel::<PagesData>(initial_data);

    // * Spawn worker ONCE
    tokio::spawn(async move {
        start_worker_handler("sclera:jobs", rx).await;
    });

    socket_receiver_loop_handler(receiver, tx, sender).await;

    println!("❌ WebSocket disconnected");
}

