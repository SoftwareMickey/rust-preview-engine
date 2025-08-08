use std::{ fs, io::Result, path::Path };

use crate::{
    preview::{
        handle_page_overwrite::handle_page_overwrite, 
        route_update::update_routes_file
    }, 
    types::PageData, 
    utils::install_commands::installation_commands
};

// pub async fn scaffold_new_react_project( 
//     project_path: &str, 
//     page: Vec<PageData>, 
//     project_name: &str, user_id : &str, site_name : &str) -> Result<()> {

//     let path = Path::new(project_path);

//     if path.exists() {
//         // * Check if it's not empty
//         let is_empty = fs::read_dir(path)?.next().is_none();

//         if is_empty {
//             println!("Path exists but is empty. Proceeding to scaffold...");
//             installation_commands(&project_path, &project_name, user_id, site_name).await?;
//         } else {

//             let project_dir = format!("{}/{}", project_path, project_name);
//             for found_page in page {
//                 let received_page = found_page;

//                 // * Create a safe reference to the name
//                 let page_name = received_page
//                     .name
//                     .as_ref()
//                     .map(String::as_str)
//                     .unwrap_or("default");

//                 let _ = update_routes_file(&project_dir, &received_page).await;
//                 handle_page_overwrite(&page_name, &project_dir, user_id, &received_page).await?;
//             }

//             // * Optionally, return early or handle differently
//             return Ok(());
//         }
//     } else {
//         println!("Creating base path: {}", project_path);
//         fs::create_dir_all(path)?;
//     }

//     Ok(())
// }


pub async fn scaffold_new_react_project(
    project_path: &str,
    page: Vec<PageData>,
    project_name: &str,
    user_id: &str,
    site_name: &str
) -> Result<()> {
    let path = Path::new(project_path);

    if !path.exists() {
        println!("Creating base path: {}", project_path);
        let path_creation = fs::create_dir_all(path);
        match path_creation {
            Ok(_) => {
                println!("Base path created successfully...")
            }
            Err(e) => {
                eprintln!("Failed to create base path : {:?}", e)
            }
        }
    }else{
        println!("PATH {:?} exists", path)
    }

    println!("Checking if path {:?} is empty", path);
    let is_empty = fs::read_dir(path)?.next().is_none();

    if is_empty {
        println!("Path is empty. Proceeding to scaffold...");
        let install_result = installation_commands(project_path, project_name, user_id, site_name).await;

        match install_result {
            Ok(_) => {
                println!("Installation succceeded...")
            }
            Err(e) => {
                eprintln!("Failed to install : {:?}", e)
            }
        }
    } else {
        // let project_dir = format!("{}/{}", project_path, project_name);
        let project_dir = format!("{}", project_path);
        for received_page in page {
            let page_name = received_page
                .name
                .as_ref()
                .map(String::as_str)
                .unwrap_or("default");

            let _ = update_routes_file(&project_dir, &received_page).await;
            handle_page_overwrite(&page_name, &project_dir, user_id, &received_page).await?;
        }
    }

    Ok(())
}


// pub async fn scaffold_new_react_project( 
//     project_path: &str, 
//     page: Vec<PageData>, 
//     project_name: &str, 
//     user_id: &str, 
//     site_name: &str
// ) -> Result<()> {
//     let path = Path::new(project_path);

//     if path.exists() {
//         let is_empty = fs::read_dir(path)?.next().is_none();

//         if is_empty {
//             println!("Path exists but is empty. Proceeding to scaffold...");
//             installation_commands(project_path, project_name, user_id, site_name).await?;
//         } else {
//             let project_dir = format!("{}/{}", project_path, project_name);
//             for received_page in page {
//                 let page_name = received_page
//                     .name
//                     .as_ref()
//                     .map(String::as_str)
//                     .unwrap_or("default");

//                 let _ = update_routes_file(&project_dir, &received_page).await;
//                 handle_page_overwrite(&page_name, &project_dir, user_id, &received_page).await?;
//             }

//             return Ok(()); // Return early if already scaffolded
//         }
//     } else {
//         println!("Creating base path: {} & Installing", project_path);
//         fs::create_dir_all(path)?;
        
//         // 🔥 You forgot to scaffold here if dir didn't exist
//         installation_commands(project_path, project_name, user_id, site_name).await.expect("Failed To Install...");

//         println!("Installation...done!")
//     }

//     Ok(())
// }
