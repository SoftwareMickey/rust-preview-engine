use serde::Deserialize;
use std::collections::HashMap;

#[allow(dead_code)]
#[derive(Deserialize, Debug, Clone)]
pub struct Element {
    #[serde(rename = "type")]
    pub element_type: String,
    pub id: String,
    pub parent_id: Option<String>,
    pub children: Option<Vec<String>>,
    pub styles: Option<HashMap<String, String>>,
    pub props: Option<HashMap<String, String>>, // * Add this line
}


#[derive(Deserialize, Debug, Clone)]
pub struct Page {
    pub name: String,
    pub path: String, // e.g., "/", "/about"
    pub elements: Vec<Element>,
}

#[allow(dead_code)]
#[derive(Deserialize)]
pub struct SitePreviewRequest {
    pub pages: Vec<Page>,
}

use serde::Serialize;

#[derive(Serialize)]
pub struct RenderedPage {
    pub path: String,
    pub html: String,
}

#[derive(Serialize)]
pub struct SiteRenderResponse {
    pub pages: Vec<RenderedPage>,
}

#[derive(Serialize)]
pub struct  ReactPreviewResponse{
    pub component: String,
    pub name: String,
}