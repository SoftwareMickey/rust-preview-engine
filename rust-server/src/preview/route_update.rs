#![allow(unused_assignments)]

use std::path::PathBuf;
use regex::Regex;
use tokio::fs;

use crate::types::PageData;

pub async fn update_routes_file(project_dir: &str, page: &PageData) -> Result<(), Box<dyn std::error::Error>> {
    
    println!("UPDATING ROUTES FOR {:?}", page.name.clone().unwrap_or_default());

    let routes_path = PathBuf::from(project_dir).join("src").join("Routes.jsx");

    println!("Routes Path {:?}", routes_path);

    if !routes_path.exists() {
        return Ok(());
    }

    let mut content = fs::read_to_string(&routes_path).await?;

    let raw_path = page.path.clone().unwrap_or_else(|| "/".to_string());
    let component_path = raw_path.trim_start_matches('/').replace(" ", "");

    let component_name = page.name.clone().unwrap_or_else(|| component_path.clone());
    // let component_name = component_name[..1].to_uppercase() + &component_name[1..];

    let component_name = if let Some(first) = component_name.chars().next() {
        format!("{}{}", first.to_uppercase(), &component_name[first.len_utf8()..])
    } else {
        "Page".to_string()
    };



    // * Import logic
    let import_re = Regex::new(&format!(
        r#"import\s+.*{}.*from\s+['"]\.\/pages/{}.*['"]"#,
        component_name, component_name
    ))?;


    if !import_re.is_match(&content) {
        let all_imports_re = Regex::new(r"(?m)^import\s.*from\s.*;?\s*$")?;

        let import_line = format!("import {} from './pages/{}';\n", component_name, component_name);

        if let Some(last_import) = all_imports_re.find_iter(&content).last() {
            // * Insert after the last import (default case)
            content.insert_str(last_import.end(), &import_line);
        } else {
            //*  */ No imports at all, insert at the top
            content = format!("{}\n{}", import_line, content);
        }
    }


    // * Inject route in children
    let children_re = Regex::new(r"children\s*:\s*\[(?P<block>[\s\S]*?)\n\s*\]")?;

    let mut already_exists = false;

    if let Some(cap) = children_re.captures(&content) {
        let children_block = cap.name("block").unwrap().as_str();
        let route_exists_re = Regex::new(&format!(r#"path:\s*['"]{}['"]"#, component_path))?;

        already_exists = route_exists_re.is_match(children_block);

        if already_exists {
            return Ok(());
        }

        let route_line = format!("\n        {{ path: '{}', element: <{} /> }},", component_path, component_name);

        let updated_block = format!("children: [{}\n    ]", children_block.to_owned() + &route_line);
        content = children_re.replace(&content, updated_block.as_str()).to_string();
    } else {
        println!("❌ Could not find children array to insert route.");
        return Ok(());
    }

    // * Write back
    fs::write(routes_path, content).await?;

    println!("ROUTES FOR {:?} UPDATED....", page.name.clone().unwrap_or_default());

    Ok(())
}

