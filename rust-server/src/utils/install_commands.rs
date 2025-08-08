#![allow(non_snake_case)]

use std::{env, thread, time::Duration};
use std::path::Path;
use std::io::{self, Result};
use std::process::Command;
use dotenvy::dotenv;

use crate::utils::{
    root_layout_builder::{
        httpCreateRootLayout, 
        httpCreateRoutesLayout, 
        httpMergeLayoutIntoApp
    }, 
    tailwind::tailwind_installation_command, 
    vite_config::httpCreateViteConfig
};

pub fn run_command(command: &str, args: &[&str], cwd: &str) -> io::Result<()> {
    println!("🛠️ Running: {} {}", command, args.join(" "));
    let status = Command::new(command)
        .args(args)
        .current_dir(cwd)
        .status()?;

    if status.success() {
        println!("✅ Command succeeded: {}", command);
        Ok(())
    } else {
        eprintln!("❌ Command failed: {} with status: {}", command, status);
        Err(io::Error::new(
            io::ErrorKind::Other,
            format!("Command {} failed with status {}", command, status),
        ))
    }
}

// * Wait for a file to exist
fn wait_for_file(path: &Path, max_retries: u32, delay_secs: u64) -> bool {
    for _ in 0..max_retries {
        if path.exists() {
            return true;
        }
        thread::sleep(Duration::from_secs(delay_secs));
        println!("Waiting for package.json");
    }
    false
}

pub async fn installation_commands(
    project_path: &str, 
    project_name: &str, 
    user_id: &str, 
    site_name: &str
) -> Result<()> {

    println!("Installing...");

    dotenv().ok();
    let NPX_PATH = env::var("NPX_PATH").expect("NPX PATH MUST BE SET");
    let NPM_PATH = env::var("NPM_PATH").expect("NPM PATH MUST BE SET");

    println!("✅ Using NPX: {}", NPX_PATH);
    println!("✅ Using NPM: {}", NPM_PATH);

    let vite_project_dir = Path::new(project_path.trim()); // * Only use what's passed
    let vite_project_path_str = vite_project_dir.to_str().unwrap_or_default();
    


    // Run create-vite
    let create_vite = run_command(
        &NPX_PATH,
        &["--yes", "create-vite@latest", ".", "--template", "react", "--", "--no-git", "--no-install"],
        project_path.trim(),
    );

    match create_vite {
        Ok(_) => {
            println!("✅ create-vite finished");
        }
        Err(e) => {
            eprintln!("❌ create-vite failed: {:?}", e);
            return Err(e);
        }
    }

    // Wait for package.json to appear
    let pkg_path = vite_project_dir.join("package.json");
    println!("⏳ Waiting for package.json at {:?}", pkg_path);
    let file_ready = wait_for_file(&pkg_path, 50, 1); // retry 10 times, 1 second apart

    if !file_ready {
        return Err(io::Error::new(
            io::ErrorKind::TimedOut,
            "Timed out waiting for package.json to be created.",
        ));
    }

    // NPM install commands
    let commands = vec![
        (
            &NPM_PATH,
            vec!["install", "vite", "@vitejs/plugin-react", "--save-dev"],
            vite_project_path_str,
        ),
        (
            &NPM_PATH,
            vec!["install", "react-router-dom", "react-redux", "@reduxjs/toolkit"],
            vite_project_path_str,
        ),
        (
            &NPM_PATH,
            vec!["install", "tailwindcss", "@tailwindcss/vite"],
            vite_project_path_str,
        ),
        (
            &NPM_PATH,
            vec!["install"],
            vite_project_path_str,
        ),
    ];

    for (cmd, args, dir) in commands {
        if let Err(e) = run_command(cmd, &args, dir.trim()) {
            eprintln!("❌ Failed to execute command: {:?} in {:?}: {:?}", cmd, dir, e);
        }
    }

    let project_dir = std::env::current_dir()?.join("sclera_builds").join(site_name);
    let project_path = project_dir.to_str().unwrap_or_default().to_string();

    println!("INSTALLATION PATH : {:?}", project_path);

    if !Path::new(&project_path).exists() {
        println!("VITE PATH : {:?}", vite_project_dir);
        panic!("❌ Project path does not exist: {}", project_path);
    }

    // * Async helpers
    tailwind_installation_command(&project_path, project_name).await.expect("FAILED TO INSTALL TAILWIND");
    httpCreateRootLayout(&project_path, project_name).await.expect("FAILED TO CREATE ROOT LAYOUT");
    httpCreateRoutesLayout(&project_path, project_name, user_id, site_name).await.expect("ROUTES CREATION FAILED");
    httpMergeLayoutIntoApp(&project_path, project_name).await.expect("LAYOUT MERGING FAILED");
    httpCreateViteConfig(&project_path, project_name, user_id, site_name).await.expect("CREATE VITE FAILED");

    println!("🎉 Project scaffolded and dependencies installed at: {}", project_path);
    println!("🚀 Finished all install commands");

    Ok(())
}


// #![allow(non_snake_case)]

// use std::{env, io::{self, Result}, path::Path, process::{Command}};
// use dotenvy::dotenv;

// use crate::utils::{
//     root_layout_builder::{
//         httpCreateRootLayout, 
//         httpCreateRoutesLayout, 
//         httpMergeLayoutIntoApp
//     }, 
//     tailwind::tailwind_installation_command, 
//     vite_config::httpCreateViteConfig
// };

// #[allow(non_snake_case)]

// pub fn run_command(command: &str, args: &[&str], cwd: &str) -> io::Result<()> {
//     println!("🛠️ Running: {} {}", command, args.join(" "));
//     // println!("🛠️ Running: {} {} (cwd: {})", command, args.join(" "), cwd);
//     let status = Command::new(command)
//         .args(args)
//         .current_dir(cwd)
//         .status()?;

//     if status.success() {
//         println!("✅ Command succeeded: {}", command);
//         Ok(())
//     } else {
//         eprintln!("❌ Command failed: {} with status: {}", command, status);
//         Err(io::Error::new(
//             io::ErrorKind::Other,
//             format!("Command {} failed with status {}", command, status),
//         ))
//     }
// }


// pub async fn installation_commands(
//     project_path: 
//     &str, project_name: &str, user_id : &str, site_name : &str) -> Result<()> {

//     println!("Installing...");

//     dotenv().ok();
//     let NPX_PATH = env::var("NPX_PATH").expect("NPX PATH MUST BE SET");
//     let NPM_PATH = env::var("NPM_PATH").expect("NPM PATH MUST BE SET");

//     println!("✅ Using NPX: {}", NPX_PATH);
//     println!("✅ Using NPM: {}", NPM_PATH);

//     println!("INSTALLATION PROJECT PATH : {:?}", project_path);

//     let vite_project_dir = Path::new(project_path).join(project_name.trim());
//     let vite_project_path_str = vite_project_dir.to_str().unwrap_or_default();

//     //* */ Commands to run in order
//     let commands = vec![
//         (
//             &NPX_PATH,
//             vec!["--yes", "create-vite@latest", project_name, "--template", "react"],
//             project_path.trim(),
//         ),
//         (
//             &NPM_PATH,
//             vec!["install", "vite", "@vitejs/plugin-react", "--save-dev"],
//             vite_project_path_str,
//         ),
//         (
//             &NPM_PATH,
//             vec!["install", "react-router-dom", "react-redux", "@reduxjs/toolkit"],
//             vite_project_path_str,
//         ),
//         (
//             &NPM_PATH,
//             vec!["install", "tailwindcss", "@tailwindcss/vite"],
//             vite_project_path_str,
//         ),
//         (
//             &NPM_PATH,
//             vec!["install"],
//             vite_project_path_str,
//         ),
//     ];

//     // * Loop and execute
//     for (cmd, args, dir) in commands {
//         let run_cmd = run_command(cmd, &args, dir.trim());
//         match run_cmd {
//             Ok(_) => {
//                 println!("Commandd executed successfully")
//             }
//             Err(e) => {
//                 eprintln!("Failed to execute command : {:?}", e)
//             }
//         }
//     }

//     // Async helper functions after npm installs
//     tailwind_installation_command(project_path, project_name).await?;
//     httpCreateRootLayout(project_path, project_name).await?;
//     httpCreateRoutesLayout(project_path, project_name, user_id, site_name).await?;
//     httpMergeLayoutIntoApp(project_path, project_name).await?;
//     httpCreateViteConfig(project_path, project_name, user_id, site_name).await?;

//     println!("🎉 Project scaffolded and dependencies installed at: {}", vite_project_path_str);
//     println!("🚀 Finished all install commands");

//     Ok(())
// }
