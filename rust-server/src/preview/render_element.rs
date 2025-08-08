use std::collections::{ HashMap };

use crate::{
    constants::tag::{ SELF_CLOSING_TAGS, TAG_MAP }, preview::{
        convert_to_tailwind::convert_style_to_tailwind, 
        find_element::find_element_by_id, 
        font_family::font_family_handler
    }, 
    render_helpers::handle_element_styles::{generate_element_props, handle_element_styles}, 
    types::{CanvasElement, Style}
};

pub fn render_element(
    page_elements : &Vec<CanvasElement>,
    tree : &Vec<CanvasElement>,
    el: &CanvasElement,
    uses: &mut HashMap<String, bool>,
    project_dir: &str,
    user_id: &str,
) -> String {

    println!("RENDERING...");

    let mut rstyles = el.styles.as_ref().unwrap_or(&Style::default()).to_hashmap();

    let ( _, height ) = handle_element_styles(&el, &rstyles);
    let mut tag = TAG_MAP.get(&el.r#type).cloned().unwrap_or("div".to_string());

    // * React router Link
    if el.r#type == "link" {
        tag = "Link".to_string();
        uses.insert("Link".to_string(), true);
    }

    let _ = font_family_handler(project_dir, user_id);

    // * Clone and mutate styles
    let styles = el.styles.clone().unwrap_or_default();

    let is_top_level_section = el.r#type == "frame" || el.r#type == "button";
    if !is_top_level_section {
        rstyles.remove("height");
    }

    let tailwind_class = convert_style_to_tailwind(
        &styles, 
        &el.r#type, height);

    // * Build opening tag
    let mut open_tag = format!("<{}", tag);
    
    if let Some(class_name) = tailwind_class {
        if !class_name.is_empty() {
            open_tag += &format!(" className=\"{}\"", class_name);
        }
    }

    open_tag += &generate_element_props(&el);

    if SELF_CLOSING_TAGS.contains(&tag) {
        open_tag += " />";
        return open_tag;
    }

    open_tag += ">";

    // * Inner text and children
    let mut inner = String::new();

    println!("ELEMENT : {:?}", el);
    // If element has no children, treat it as a leaf node
    let is_leaf = el.children.is_none() || el.children.as_ref().unwrap().is_empty();

    if is_leaf {
        if let Some(props) = &el.props {
            if let Some(text) = props.get("text") {
                println!("LEAF TEXT : {:?}", text);
                inner += text.as_str().expect("TEXT");
            }
        }
    } else {
        if let Some(children) = &el.children {
            for child in children {
                println!("🔍 Looking for child ID: {}", child);

                if let Some(found) = find_element_by_id(&page_elements, child) {
                    println!("✅ Found element: {:?}", found);
                    inner += &render_element(page_elements, tree, found, uses, project_dir, user_id);
                }
            }
        }
    }


    format!("{}{}</{}>", open_tag, inner, tag)

}
