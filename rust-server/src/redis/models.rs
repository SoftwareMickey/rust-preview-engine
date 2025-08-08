use serde::{Deserialize, Serialize};
use crate::types::{PageData};

#[derive(Serialize)]
pub struct OutgoingSocketMessage {
    pub event: String,
    pub data: Vec<PageData>,
    pub path : String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkerResponse {
    pub event: String,
    pub message: String,
    pub status_code: i32,
    pub data: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Job {
    pub job_type: String,
    pub site_name: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct PreviewConfig {
    pub preview_path: String,
}