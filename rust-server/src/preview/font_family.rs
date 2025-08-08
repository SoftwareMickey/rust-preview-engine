use std::{io::Result, path::PathBuf};
use tokio::fs;
use regex::Regex;
use std::collections::HashSet;

static USED_FONTS: &[&str] = &[
    "Lily Script One",
    "Roboto",
    "Open Sans",
    "JetBrains Mono",
];

fn slugify_font_name(font: &str) -> String {
    format!("font-{}", font.to_lowercase().replace(" ", "-"))
}

fn get_fallback(font: &str) -> &'static str {
    if font.to_lowercase().contains("serif") {
        "serif"
    } else if font.to_lowercase().contains("mono") {
        "monospace"
    } else if font.to_lowercase().contains("script") {
        "cursive"
    } else {
        "sans-serif"
    }
}

pub async fn font_family_handler(project_path: &str, user_id: &str) -> Result<()> {
    if project_path.is_empty() {
        return Ok(());
    }

    let fonts: HashSet<_> = USED_FONTS.iter().copied().collect();
    if fonts.is_empty() {
        return Ok(());
    }

    // * Build Google Fonts URL
    let font_url = format!(
        "https://fonts.googleapis.com/css2?{}&display=swap",
        fonts
            .iter()
            .map(|f| format!("family={}{}", f.replace(" ", "+"), ":wght@100;300;400;500;600;700"))
            .collect::<Vec<_>>()
            .join("&")
    );

    let head_tag = format!(r#"<link href="{font_url}" rel="stylesheet">"#);

    // * === Handle index.html

    let index_html_path = PathBuf::from(project_path).join("index.html");
    if index_html_path.exists() {
        let mut html = fs::read_to_string(&index_html_path).await?;
        if !html.contains(&font_url) {
            let re = Regex::new(r"(?i)</head>").unwrap();
            html = re
                .replace(&html, format!("  {}\n</head>", head_tag))
                .to_string();
            fs::write(&index_html_path, html).await?;

            println!("✅ Google Fonts <link> inserted into index.html for user {}", user_id);
        } else {
            println!("ℹ️ Font link already exists for user {}", user_id);
        }
    } else {
        println!("⚠️ index.html not found.");
    }

    // * === Handle index.css

    let css_path = PathBuf::from(project_path).join("src").join("index.css");
    if css_path.exists() {
        let css = fs::read_to_string(&css_path).await?;
        let mut new_styles = String::new();

        for font in &fonts {
            let class_name = slugify_font_name(font);
            let fallback = get_fallback(font);
            let css_block = format!(
                ".{} {{\n  font-family: '{}', {};\n}}\n",
                class_name, font, fallback
            );

            if !css.contains(&format!(".{}", class_name)) {
                new_styles += &css_block;
                new_styles += "\n";
            }
        }

        if !new_styles.trim().is_empty() {
            let comment = "/* === Generated Font Classes === */\n";
            fs::write(&css_path, format!("{}\n{}", css, comment.to_string() + &new_styles)).await?;
            println!("✅ Font classes inserted for user {}", user_id);
        } else {
            println!("ℹ️ Font classes already present for user {}", user_id);
        }
    } else {
        println!("⚠️ index.css not found.");
    }

    Ok(())
}
