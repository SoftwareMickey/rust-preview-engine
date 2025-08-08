// use axum::{extract::ws::{Message}};
// use crate::{sockets::models::{SocketEvent, SocketModel, SocketRespone, Status}, types::PagesData};
// use tokio::sync::mpsc::UnboundedSender;


// pub async fn handle_page_update(
//     tx : &UnboundedSender<Message>, 
//     data_received : &PagesData){

//     let response = SocketRespone{
//         event : SocketEvent::PageUpdate,
//         message : "Page update sucessfully".into(),
//         status_code : 200,
//         status : Status::Success,
//         data : Some(serde_json::json!({ "pages" : data_received.pages}))
//     };

//     if let Ok(json) = serde_json::to_string(&response){
//         let _ = tx.send(Message::Text(json.into()));   
//     }
// }

// pub async fn handle_socket_connection(
//     tx: &UnboundedSender<Message>,
//     data: SocketModel,
// ) {
//     let response = SocketRespone {
//         event: SocketEvent::Connect,
//         message: "Socket connected".into(),
//         status_code: 200,
//         status: Status::Success,
//         data: Some(serde_json::json!({
//             "userId": data.user_id,
//             "siteName": data.site_name,
//         })),
//     };

//     if let Ok(json) = serde_json::to_string(&response) {
//         let _ = tx.send(Message::Text(json.into()));
//     }
// }


// pub async fn handle_socket_disconnection(
//     tx : &UnboundedSender<Message>, 
//     data : SocketModel){
//     let response = SocketRespone{
//         event : SocketEvent::Connect,
//         message : "Socket disconnected".into(),
//         status_code : 500,
//         status : Status::Error,
//         data : Some(serde_json::json!({ "userId " : data.user_id, "siteName" : data.site_name}))
//     };

//     if let Ok(json) = serde_json::to_string(&response){
//         let _ = tx.send(Message::Text(json.into()));
//     }
// }