use std::collections::HashMap;

use serde::{ Serialize, Deserialize };
use serde_json::Value;

#[allow(non_snake_case)]
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Style {
    pub position: Option<String>,
    pub layout: Option<String>,
    pub text_align: Option<String>,
    pub background_color: Option<String>,
    pub top: Option<String>,
    pub left: Option<String>,
    pub width: Option<String>,
    pub height: Option<String>,
    pub font_weight: Option<String>,
    pub font_size: Option<String>,
    pub font_family: Option<String>,
    pub color: Option<String>,
    pub border_style: Option<String>,
    pub border_color:Option<String>,
    pub border_radius: Option<String>,
    pub border_width: Option<String>,
    pub pageHeight : Option<i32>,
    pub pageWidth : Option<i32>,
}

impl Style {
    pub fn to_hashmap(&self) -> HashMap<String, String> {
        let map = HashMap::new();
        // map.insert("position".to_string(), self.position.clone());
        // map.insert("layout".to_string(), self.layout.clone());
        // map.insert("textAlign".to_string(), self.text_align.clone());
        // map.insert("backgroundColor".to_string(), self.background_color.clone());
        // map.insert("top".to_string(), self.top.clone());
        // map.insert("left".to_string(), self.left.clone());
        // map.insert("width".to_string(), self.width.clone());
        // map.insert("height".to_string(), self.height.clone());
        // map.insert("fontWeight".to_string(), self.font_weight.clone());
        // map.insert("fontSize".to_string(), self.font_size.clone());
        // map.insert("fontFamily".to_string(), self.font_family.clone());
        // map.insert("color".to_string(), self.color.clone());
        // map.insert("borderStyle".to_string(), self.border_style.clone());
        // map.insert("borderColor".to_string(), self.border_color.clone());
        // map.insert("borderRadius".to_string(), self.border_radius.clone());
        // map.insert("borderWidth".to_string(), self.border_width.clone());
        map
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CanvasElement {
    pub id: String,
    pub r#type: String,
    pub props: Option<HashMap<String, Value>>,
    pub children: Option<Vec<String>>,
    pub parent_id : Option<String>,
    pub styles : Option<Style>
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PageData {
    pub id: String,
    pub path : Option<String>,
    pub name: Option<String>,
    pub elements: Vec<CanvasElement>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PagesData {
    pub pages : Vec<PageData>,
    pub user_id : Option<String>,
    pub user_name : Option<String>,
    pub site_name : Option<String>,
    pub project_id : Option<String>,
    pub project_name : Option<String>
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PagesResponseData {
    pub pages : Vec<PageData>
}
