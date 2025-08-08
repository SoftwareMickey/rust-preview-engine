use std::collections::{HashMap, HashSet};

use crate::{preview::{build_tree::build_element_tree, render_element::render_element}, types::{CanvasElement, PageData}, utils::capitalize_first::capitalize_first};

pub fn http_generate_page_jsx(page: &PageData, project_dir: &str, user_id: &str) -> String {

    let page_elements = &page.elements;
    
    let tree = build_element_tree(page.elements.clone());

    let mut uses = HashMap::new();
    uses.insert("Link".to_string(), false);

    let all_child_ids : HashSet<String> = tree
        .iter()
        .flat_map(|el| el.children.clone().unwrap_or_default())
        .collect();

    println!("FOUND CHILD IDS : {:?}", all_child_ids);

    let top_level_elements : Vec<&CanvasElement> = tree
        .iter()
        .filter(|el| !all_child_ids.contains(&el.id))
        .collect();

    let body = top_level_elements
        .iter()
        .map(|el| render_element(page_elements, &tree, el, &mut uses, project_dir, user_id))
        .collect::<Vec<String>>()
        .join("\n");

    let component_name = page
        .name
        .clone()
        .unwrap_or("Untitled".to_string())
        .replace(" ", "");

    let imports = if *uses.get("Link").unwrap_or(&false) {
        r#"import { Link } from "react-router-dom";\n"#.to_string()
    } else {
        "".to_string()
    };

    return  format!(
        r#"{imports}
export default function {component_name}() {{
    return (
{indented_body}
    );
}}"#,
        imports = imports.trim(),
        component_name = capitalize_first(&component_name),
        indented_body = indent_lines(&body, 3)
    ).trim_start().to_string();
}

fn indent_lines(code: &str, level: usize) -> String {
    let indent = "  ".repeat(level);
    code.lines()
        .map(|line| format!("{indent}{line}", indent = indent))
        .collect::<Vec<String>>()
        .join("\n")
}
