use std::path::Path;

use axum::{response::IntoResponse, Json};
use hyper::StatusCode;
use tokio::fs;

use crate::{preview::helpers::scaffold_new_react_project, types::{PagesData, PagesResponseData}};

pub async fn receive_preview(Json(payload) : Json<PagesData>) -> 
Result<impl IntoResponse, StatusCode> {

    println!("Executing preview....");

    let rendered_pages = payload.pages;
    let user_id = payload.user_id.unwrap();
    let site_name = payload.site_name.unwrap();

    println!("USER ID : {:?}", user_id);
    println!("SITE NAME : {:?}", site_name);
    
    let project_name = payload.project_name.unwrap_or_default();

    // let project_path = format!("./tmp/{}", payload.project_id.unwrap_or_default());
    // let path = std::env::current_dir().unwrap();
    // let full_path = path.join("sclera_builds").join(&project_name);
    // let full_path = path.join("sclera_builds");
    // let project_path = full_path.to_str().unwrap_or_default().to_string();

    let root_dir = std::env::var("PROJECT_ROOT").unwrap_or_else(|_| "./sclera_builds".to_string());
    let project_path = format!("{}/{}", root_dir, site_name);

    println!("PROJECT PATH : {:?}", project_path);

    if !Path::new(&project_path).exists(){
        let _ = fs::create_dir_all(&project_path).await;
    }

    let _ = scaffold_new_react_project(
        &project_path, 
        rendered_pages.clone(), 
        &project_name,
        &user_id,
        &site_name
    ).await;

    Ok(Json(PagesResponseData {
        pages : rendered_pages
    }))
}