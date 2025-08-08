use serde::{Deserialize, Serialize};
use crate::types::PagesData;

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum SocketEvent {
    PageUpdate,
    Connect
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SocketModel {
    pub event : SocketEvent,
    pub user_id : String, 
    pub site_name : String,
    pub payload : Option<PagesData>
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Status {
    Success,
    Error
}

#[derive(Debug, Serialize, Deserialize)]
pub struct  SocketRespone {
    pub event : SocketEvent,
    pub message : String,
    pub status_code : i32,
    pub status : Status,
    pub data : Option<serde_json::Value>
}