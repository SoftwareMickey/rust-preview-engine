use axum::{
    extract::Json,
    http::{HeaderMap, HeaderValue, StatusCode},
    response::IntoResponse,
};

use dotenvy::dotenv;
use reqwest::Client;
use serde_json::{json, Value};
use std::env;

#[allow(non_snake_case)]
pub async fn react_preview(Json(payload): Json<Value>) -> impl IntoResponse {
    dotenv().ok();

    let REACT_GEN_URL = env::var("REACT_GEN_URL").expect("REACT GEN URL MUST BE SET");
    println!("URL : {:?}", REACT_GEN_URL);

    let client = Client::new();
    let res = client
        .post(format!("{}/api/v1/generate", REACT_GEN_URL))
        .json(&payload)
        .send()
        .await;

    match res {
        Ok(response) => {
            if !response.status().is_success() {
                return (
                    StatusCode::BAD_REQUEST,
                    Json(json!({ "error": "Failed to generate zip" })),
                )
                    .into_response();
            }

            // * Clone header values before .bytes() to avoid borrow/move issue
            let content_disposition = response
                .headers()
                .get("content-disposition")
                .and_then(|v| v.to_str().ok())
                .unwrap_or("attachment; filename=\"sclera-app.zip\"")
                .to_string();

            let content_type = response
                .headers()
                .get("content-type")
                .and_then(|v| v.to_str().ok())
                .unwrap_or("application/zip")
                .to_string();

            match response.bytes().await {
                Ok(bytes) => {
                    let mut headers = HeaderMap::new();
                    headers.insert("Content-Type", HeaderValue::from_str(&content_type).unwrap());
                    headers.insert(
                        "Content-Disposition",
                        HeaderValue::from_str(&content_disposition).unwrap(),
                    );
                    headers.insert(
                        "Content-Length",
                        HeaderValue::from_str(&bytes.len().to_string()).unwrap(),
                    );

                    (StatusCode::OK, headers, bytes).into_response()
                }
                Err(e) => (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(json!({ "error": format!("Read failed: {}", e) })),
                )
                    .into_response(),
            }
        }
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "error": format!("Request failed: {}", e) })),
        )
            .into_response(),
    }
}
