use std::{collections::{HashMap, HashSet}, sync::Mutex};

use once_cell::sync::{Lazy};

pub static USED_FONTS: Lazy<Mutex<HashSet<String>>> = Lazy::new(|| Mutex::new(HashSet::new()));

pub static TAG_MAP: Lazy<HashMap<String, String>> = Lazy::new(|| {
    let mut m = HashMap::new();
    m.insert("text".to_string(), "p".to_string());
    m.insert("frame".to_string(), "section".to_string());
    m.insert("image".to_string(), "img".to_string());
    m.insert("button".to_string(), "button".to_string());
    m.insert("link".to_string(), "Link".to_string());

    m
});

pub static SELF_CLOSING_TAGS: Lazy<HashSet<String>> = Lazy::new(|| {
    ["img", "input", "br", "hr"]
        .iter()
        .map(|s| s.to_string())
        .collect()
});