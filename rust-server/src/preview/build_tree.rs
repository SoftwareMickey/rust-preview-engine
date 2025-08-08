use std::collections::HashMap;
use crate::types::CanvasElement;

pub fn build_element_tree(mut elements: Vec<CanvasElement>) -> Vec<CanvasElement> {
    let mut element_map: HashMap<String, CanvasElement> = HashMap::new();
    let mut roots: Vec<CanvasElement> = Vec::new();

    // * Clear existing children before tree-building to avoid duplication
    for el in elements.iter_mut() {
        el.children = None;
        element_map.insert(el.id.clone(), el.clone());
    }

    // * Collect parent-child relationships
    let mut links: Vec<(String, String)> = Vec::new();
    
    for el in element_map.values() {
        if let Some(parent_id) = &el.parent_id {
            links.push((el.id.clone(), parent_id.clone()));
        }
    }

    // * Build tree
    for (child_id, parent_id) in links {
        if let Some(parent) = element_map.get_mut(&parent_id) {
            parent.children.get_or_insert_with(Vec::new).push(child_id);
        }
    }

    // *  Get roots
    for el in element_map.values() {
        if el.parent_id.is_none() {
            roots.push(el.clone());
        }
    }

    roots
}
