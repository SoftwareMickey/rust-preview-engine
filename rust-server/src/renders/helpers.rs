use std::collections::HashMap;

pub fn camel_to_kebab(input: &str) -> String {
    let mut result = String::new();
    for (i, c) in input.chars().enumerate() {
        if c.is_uppercase() {
            if i != 0 {
                result.push('-');
            }
            result.push(c.to_ascii_lowercase());
        } else {
            result.push(c);
        }
    }
    result
}

pub fn parse_px(value: &str) -> Result<f32, std::num::ParseFloatError> {
    value.trim_end_matches("px").parse::<f32>()
}

pub fn inline_style(style: &Option<HashMap<String, String>>) -> String {
    style.as_ref().map_or(String::new(), |s| {
        s.iter()
         .map(|(k, v)| format!("{}:{};", camel_to_kebab(k), v))
         .collect::<Vec<String>>()
         .join(" ")
    })
}