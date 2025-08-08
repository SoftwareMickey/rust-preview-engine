use axum::extract::ws::{Message, WebSocket};
use futures::{stream::{SplitSink, SplitStream}, SinkExt};
use futures_util::{StreamExt};
use serde_json::json;

use crate::{
    sockets::{models::{SocketEvent, SocketModel}, 
    update_event::SOCKET_UPDATE_EVENT_HANDLER}, types::PagesData
};

use tokio::sync::watch::Sender;

pub async fn socket_receiver_loop_handler(
    mut receiver: SplitStream<WebSocket>,
    tx: Sender<PagesData>,
    mut sender : SplitSink<WebSocket, Message>
) {
    while let Some(Ok(msg)) = receiver.next().await {
        if let Message::Text(text) = msg {
            if let Ok(data) = serde_json::from_str::<SocketModel>(&text) {
                match data.event {
                    SocketEvent::PageUpdate => {
                        SOCKET_UPDATE_EVENT_HANDLER(tx.clone(), &mut sender, data).await;
                    }

                    SocketEvent::Connect => {
                        let connect_msg = serde_json::to_string(&json!({
                            "event" : "connect",
                            "message" : "CONNECTED TO RUST SOCKET",
                            "status" : "OK",
                            "status_code" : 200
                        })).unwrap();

                        sender.send(Message::Text(connect_msg.into()))
                            .await
                            .expect("FAILED TO EMIT CONNECT MESSAGE...");
                    }
                    // _ => println!("Unhandled event {:?}", data.event),
                }
            }
        }
    }
}
