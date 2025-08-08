use regex::Regex;

use crate::types::Style;

// * Converts a hex color to a Tailwind-compatible background class
fn get_bg_class_from_color(hex: &str) -> String {
    let re = Regex::new(r"^#([A-Fa-f0-9]{6})$").unwrap();
    if hex.is_empty() || !re.is_match(hex) {
        "bg-transparent".to_string()
    } else {
        format!("bg-[{}]", hex)
    }
}

// * Converts a font name to Tailwind font class format
fn slugify_font_name(font_name: &str) -> String {
    format!("font-{}", font_name.to_lowercase().replace(' ', "-"))
}


// * Converts a style object into a string of Tailwind classes

pub fn convert_style_to_tailwind(style: &Style, element_type: &str, height: i32) -> Option<String> {
    let mut class_names : Vec<String> = vec![];

    // * Layout & Position
    match style.position.as_deref().unwrap_or_default(){
        "absolute" => class_names.push("absolute".to_string()),
        "relative" => class_names.push("relative".to_string()),
        _ => {}
    }

    if style.layout.as_deref().unwrap_or_default() == "block" {
        class_names.push("block".to_string());
    }

    if element_type != "frame" {
        match style.text_align.as_deref().unwrap_or_default() {
            "left" => class_names.push("text-left".to_string()),
            "center" => class_names.push("text-center".to_string()),
            "right" => class_names.push("text-right".to_string()),
            "justify" => class_names.push("text-justify".to_string()),
            _ => {}
        }
    }

    // * Background color
    if !style.background_color.as_deref().unwrap_or_default().is_empty() {
        class_names.push(get_bg_class_from_color(&style.background_color.as_deref().unwrap_or_default()));
    }

    if element_type == "frame" {
        class_names.push(format!("h-[{}px]", height));
        class_names.push("w-[100vw]".to_string());
    }

    if element_type != "frame" {
        if !style.top.as_deref().unwrap_or_default().is_empty() {
            class_names.push(format!("top-[{}]", style.top.as_deref().unwrap_or_default()));
        }
        if !style.left.as_deref().unwrap_or_default().is_empty() {
            class_names.push(format!("left-[{}]", style.left.as_deref().unwrap_or_default()));
        }
        if !style.width.as_deref().unwrap_or_default().is_empty() {
            class_names.push(format!("w-[{}]", style.width.as_deref().unwrap_or_default()));
        }
        if !style.height.as_deref().unwrap_or_default().is_empty() {
            class_names.push(format!("h-[{}]", style.height.as_deref().unwrap_or_default()));
        }
    }

    // Typography
    if element_type != "frame" {
        if !style.font_weight.as_deref().unwrap_or_default().is_empty() {
            class_names.push(format!("font-[{}]", style.font_weight.as_deref().unwrap_or_default()));
        }
        if !style.font_size.as_deref().unwrap_or_default().is_empty() {
            class_names.push(format!("text-[{}]", style.font_size.as_deref().unwrap_or_default()));
        }
        if !style.font_family.as_deref().unwrap_or_default().is_empty() {
            class_names.push(slugify_font_name(&style.font_family.as_deref().unwrap_or_default()));
        }
        if !style.color.as_deref().unwrap_or_default().is_empty() {
            class_names.push(format!("text-[{}]", style.color.as_deref().unwrap_or_default()));
        }
    }

    if element_type == "button" {
        class_names.push("hover:cursor-pointer".to_string());
    }

    // Borders
    if style.border_style.as_deref().unwrap_or_default() == "solid" {
        class_names.push("border-solid".to_string());
    }
    if style.border_color.as_deref().unwrap_or_default() == "transparent" {
        class_names.push("border-transparent".to_string());
    }
    if !style.border_radius.as_deref().unwrap_or_default().is_empty() {
        class_names.push(format!("rounded-[{}]", style.border_radius.as_deref().unwrap_or_default()));
    }
    if style.border_width.as_deref().unwrap_or_default() == "1px" {
        class_names.push("border".to_string());
    }

    let joined = class_names.join(" ");

    if joined.is_empty(){
        None
    }else {
        Some(joined)
    }
}
