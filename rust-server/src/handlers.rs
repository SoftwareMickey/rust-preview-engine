use axum::{Json, response::IntoResponse, http::StatusCode};
use crate::models::{Element,SitePreviewRequest, SiteRenderResponse, RenderedPage};
use crate::renders::{button::render_button, frame::render_frame, link::render_link, text::render_text};

use std::collections::{HashMap, HashSet};

pub async fn preview(Json(payload): Json<SitePreviewRequest>) -> Result<impl IntoResponse, impl IntoResponse> {
    let mut rendered_pages = Vec::new();

    for page in payload.pages {
        match render_page(&page.name, &page.elements) {
            Ok(html) => rendered_pages.push(RenderedPage {
                path: page.path.clone(),
                html,
            }),
            Err(e) => {
                eprintln!("Error rendering page {}: {}", page.name, e);
                return Err((StatusCode::INTERNAL_SERVER_ERROR, format!("Render error: {e}")).into_response());
            }
        }
    }

    Ok(Json(SiteRenderResponse {
        pages: rendered_pages,
    }))
}

pub fn render_element(
    el: &Element,
    element_map: &HashMap<String, Element>,
    parent_width: Option<f32>,
    parent_height: Option<f32>,
) -> Result<String, String> {
     match el.element_type.as_str() {
        "frame" => render_frame(el, element_map),
        "text" => render_text(el, parent_width, parent_height),
        "button" => render_button(el, parent_width, parent_height),
        "link" => render_link(el, parent_width, parent_height),
        _ => Err(format!("Unsupported element type: {}", el.element_type)),
    }
}


fn render_page(page_name: &str, elements: &Vec<Element>) -> Result<String, String> {
    let mut element_map: HashMap<String, Element> = HashMap::new(); // ✅ Own the values
    let mut child_ids = HashSet::new();

    for el in elements {
        if el.id.is_empty() {
            return Err("Element missing 'id'".to_string());
        }
        if let Some(children) = &el.children {
            for child_id in children {
                child_ids.insert(child_id.clone());
            }
        }
        element_map.insert(el.id.clone(), el.clone()); // ✅ insert full value
    }

    let mut html = String::from(r#"<!DOCTYPE html><html class="h-full"><head>
        <link href="https://cdn.jsdelivr.net/npm/tailwindcss@2.2.19/dist/tailwind.min.css" rel="stylesheet">
        <title>"#);
    html.push_str(page_name);
    html.push_str(r#"</title></head><body class="h-full">"#);

    for el in element_map.values().filter(|e| !child_ids.contains(&e.id)) {
        match render_element(el, &element_map, None, None) {
            Ok(rendered) => html.push_str(&rendered),
            Err(e) => return Err(format!("Error rendering element {}: {}", el.id, e)),
        }
    }

    html.push_str("</body></html>");
    Ok(html)
}
