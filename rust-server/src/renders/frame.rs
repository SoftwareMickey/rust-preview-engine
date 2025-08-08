use crate::{handlers::render_element, models::Element, renders::helpers::{inline_style, parse_px}};
use std::collections::HashMap;

pub fn render_frame(
    el: &Element,
    element_map: &HashMap<String, Element>,
) -> Result<String, String> {
    let original_styles = el.styles.clone().unwrap_or_default();
    let scale_children = el.props
        .as_ref()
        .and_then(|p| p.get("scaleChildren"))
        .map(|v| v == "true" || v == "1")
        .unwrap_or(true);

    let pw = original_styles.get("width").and_then(|v| parse_px(v).ok());
    let ph = original_styles.get("height").and_then(|v| parse_px(v).ok());

    let mut frame_styles = original_styles.clone();
    frame_styles.insert("border".to_string(), "none".to_string());
    frame_styles.insert("width".to_string(), "100%".to_string());
    frame_styles.insert("height".to_string(), "1024px".to_string());
    frame_styles.insert("left".to_string(), "0".to_string());
    frame_styles.insert("top".to_string(), "0".to_string());
    frame_styles.insert("position".to_string(), "relative".to_string());

    let style = inline_style(&Some(frame_styles.clone()));
    let mut content = String::new();

    if let Some(children) = &el.children {
        for child_id in children {
            if let Some(child) = element_map.get(child_id) {
                content.push_str(&render_element(
                    child,
                    element_map,
                    if scale_children { pw } else { None },
                    if scale_children { ph } else { None },
                )?);
            } else {
                return Err(format!("Missing child element with id: {}", child_id));
            }
        }
    }

    Ok(format!(r#"<section style="{style}">{content}</section>"#))
}
