use std::collections::HashMap;

use crate::{constants::tag::USED_FONTS, types::CanvasElement};

pub fn handle_element_styles(el: &CanvasElement, rstyles : &HashMap<String, String>) -> (i32, i32) {
    let mut width = 0;
    let mut height = 0;

    if let Some(_) = &el.styles {
        if let Some(font_family) = rstyles.get("fontFamily") {
            // * Simulate font usage registry
            USED_FONTS.lock().unwrap().insert(font_family.clone());
        }

        if el.r#type == "frame" {
            let retrieved_height = rstyles.get("pageHeight").cloned().unwrap_or_default();
            height = retrieved_height.parse::<i32>().unwrap_or_default();

            let retrieved_width = rstyles.get("pageWidth").cloned().unwrap_or_default();
            width = retrieved_width.parse::<i32>().unwrap_or_default();
        }
    }

    (width, height)
}


pub fn generate_element_props(el: &CanvasElement) -> String {
    let mut result = String::new();

    if let Some(props) = &el.props {
        for (key, value) in props {
            if !["text", "width", "height", "targetId", "targetPageId", "href"].contains(&key.as_str()) {
                if el.r#type == "link" && key == "target" {
                    result += &format!(" to=\"..{}\"", value);
                } else {
                    result += &format!(" {}=\"{}\"", key, value);
                }
            }
        }
    }

    result
}
