use crate::{models::Element, renders::helpers::{inline_style, parse_px}};

pub fn render_button(
    el : &Element,
    parent_width: Option<f32>,
    parent_height: Option<f32>,
) -> Result<String, String> {

    let mut computed_styles = el.styles.clone().unwrap_or_default();

    if let Some(pw) = parent_width {
        if let Some(left) = computed_styles.get("left") {
            if let Ok(val) = parse_px(left) {
                let percent = ((val / pw) * 100.0).min(100.0);
                computed_styles.insert("left".to_string(), format!("{:.2}%", percent));
            }
        }
    }

    if let Some(ph) = parent_height {
        if let Some(top) = computed_styles.get("top") {
            if let Ok(val) = parse_px(top) {
                let percent = ((val / ph) * 100.0).min(100.0);
                computed_styles.insert("top".to_string(), format!("{:.2}%", percent));
            }
        }

        if let Some(height) = computed_styles.get("height") {
            if let Ok(v) = parse_px(height) {
                computed_styles.insert("height".to_string(), format!("{:.2}%", (v / ph) * 100.0));
            }
        }
        if let Some(font_size) = computed_styles.get("font-size") {
            if let Ok(v) = parse_px(font_size) {
                computed_styles.insert("font-size".to_string(), format!("{:.2}vw", (v / 1440.0) * 100.0));
            }
        }
    }

    computed_styles.remove("marginTop");
    computed_styles.remove("marginLeft");
    computed_styles.remove("marginRight");
    computed_styles.remove("marginBottom");
    computed_styles.insert("position".to_string(), "absolute".to_string());

    let style = inline_style(&Some(computed_styles));
    let content = el.props.as_ref()
        .and_then(|p| p.get("text"))
        .cloned()
        .unwrap_or_default();

    Ok(format!(r#"<button style="{style}">{content}</button>"#))
}

