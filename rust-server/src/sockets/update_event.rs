#![allow(non_snake_case)]

use axum::extract::ws::{Message, WebSocket};
use futures::stream::SplitSink;
use futures_util::SinkExt;

use crate::{redis::{models::{Job, OutgoingSocketMessage}, queue::queue_handler, redis::return_preview_port_to_user}, sockets::models::SocketModel, types::PagesData};

use tokio::sync::watch::Sender;

pub async fn SOCKET_UPDATE_EVENT_HANDLER(
    tx: Sender<PagesData>,
    sender : &mut SplitSink<WebSocket, Message>,
    data : SocketModel
) {
    let payload = data.payload.unwrap();

    println!("PAGE UPDATE EVENT RECEIVED....");

    let job = Job {
        job_type: "react-code-build".into(),
        site_name: "test-1".into(),
    };

    queue_handler("sclera:jobs", &job).await.expect("FAILED TO QUEUE");

    // * Update the watch channel
    let tx_messager = tx.send(payload);

    match tx_messager {
        Ok(_) => {
            println!("MESSAGE SENT TO TX...")
        }
        Err(e) => {
            println!("FAILED TO SENT MESSAFE TO TX : {:?}", e)
        }
    }

    // queue_publisher_handler().await.expect("FAILED TO NOTIFY...");f


    let latest_data = tx.borrow().clone();
    let preview_path = return_preview_port_to_user().await;

    let outgoing = OutgoingSocketMessage {
        event: String::from("page_update"),
        data: latest_data.pages,
        path : preview_path
    };

    let send_result = serde_json::to_string(&outgoing);
    match send_result {
        Ok(json) => {
            sender
                .send(Message::Text(json.into()))
                .await
                .expect("FAILED TO EMIT UPDATE EVENT.");

            println!("UPDATE EVENT SENT BACK TO USER...")
        }
        Err(e) => {
            eprintln!("FAILED TO SEND EVENT BACK : {:?}", e)
        }
    }
}
