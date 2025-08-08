use std::{io::Result, path::Path};
use tokio::fs;

use crate::{preview::page_generator::http_generate_page_jsx, types::PageData, utils::capitalize_first::capitalize_first};

pub async fn handle_page_overwrite(
    page_name: &str,
    project_dir: &str,
    user_id: &str,
    page: &PageData,
) -> Result<()> {

    println!("RECEIVED PROJECT DIR : {:?}", project_dir);

    //   let project_dir = std::env::current_dir()?.join("sclera_builds").join(site_name);
    // let project_path = project_dir.to_str().unwrap_or_default().to_string();

    let full_path = format!("{}/src/pages/{}.jsx", project_dir, capitalize_first(page_name));
    let path = Path::new(&full_path);

    println!("DERIEVED PROJECT FULL PATH : {:?}", path);

    let parent_dir = path.parent().unwrap(); // Get the directory (e.g., src/pages)

    // * Ensure the directory exists
    if !fs::try_exists(parent_dir).await.unwrap_or(false) {
        fs::create_dir_all(parent_dir).await?;
    }

    // * Generate new JSX content
    let new_page_content = http_generate_page_jsx(page, project_dir, user_id);

    let should_overwrite = if !fs::try_exists(&path).await.unwrap_or(false) {
        true
    } else {
        let existing_content = fs::read_to_string(&path).await.unwrap_or_default();
        existing_content != new_page_content
    };

    if should_overwrite {
        fs::write(&path, new_page_content).await?;
        println!("Page {:?} overwritten", capitalize_first(page_name));
    } else {
        println!("No changes for {:?}", capitalize_first(page_name));
    }

    Ok(())
}
