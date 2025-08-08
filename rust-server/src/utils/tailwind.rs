#![allow(non_snake_case)]

use std::{io::Result};

pub async fn tailwind_installation_command(project_path : &str, _project_name : &str) -> Result<()> {

    println!("📦 Installing Tailwind at path: {}", project_path);

     if !std::path::Path::new(project_path).exists() {
        println!("Path do not exist to install tailwind : {}", project_path)
    }

    httpIndexCssConfig(project_path).await.expect("FAILED TO CONFIGURE INDEX.CSS");

    println!("TAILWIND INSTALLED SUCCESSFULLY....");

    Ok(())

}


pub async fn httpIndexCssConfig(project_path : &str) -> Result<()>{

    let css_path = format!("{}/src/index.css", project_path);

    let cssLayoutConfig = format!(r#"
@import "tailwindcss";
"#).trim_start().to_string();

    std::fs::write(css_path, cssLayoutConfig).expect("FAILED TO WRITE INDEX.CSS FILE");

    Ok(())
}