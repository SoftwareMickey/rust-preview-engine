use crate::types::CanvasElement;

pub fn find_element_by_id<'a>(tree: &'a Vec<CanvasElement>, id: &str) -> Option<&'a CanvasElement> {
    for el in tree {
        if el.id == id {
            return Some(el);
        }
    }
    None
}
