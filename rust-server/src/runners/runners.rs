// #![allow(non_snake_case)]

// use std::env;
// use std::process::Command;
// use redis::{AsyncCommands};

// pub async fn _project_runner_handler(site_name : &str) {

//     dotenv().ok();
//     let NPM_PATH = env::var("NPM_PATH").expect("NPM KEY MUST BE SET");

//     let path = std::env::current_dir().unwrap();
//     let full_path = path.join("sclera_builds");
//     let project_path = full_path.to_str().unwrap_or_default().to_string();
//     let project_dir = format!("{}/{}", project_path, site_name);
//     println!("FULL PATH : {:?}", project_dir);


//     // * GET REDIS CONNECTION TO CHECK DEV SERVER PORT ACTIVE
//     let mut redis_conn = get_redis_connection().await.expect("Failed to Connect to redis");
//     let redis_key = format!("dev:site:{}", site_name);

//     // * Check if dev server is already running
//     if let Ok(json) = redis_conn.get::<_, String>(&redis_key).await {
//         if let Ok(info) = serde_json::from_str::<DevServerInfo>(&json) {
//             println!("⚠️ Dev server already running for {} on port {}", site_name, info.port);
//             return;
//         }
//     }

//     let port = get_available_port().expect("Failed to find a free port!");

//     let child = Command::new(NPM_PATH)
//         .args(["run", "dev", "--", "--port", &port.to_string(), "--host"])
//         .current_dir(project_dir)
//         .spawn()
//         .expect("Failed to start Vite dev");


//     let info = DevServerInfo {
//         port,
//         pid: child.id(),
//         last_ping: chrono::Utc::now().timestamp() as u64,
//     };

//     // * Store in Redis
//     let json = serde_json::to_string(&info).unwrap();
//     let _: () = redis_conn.set(&redis_key, json).await.unwrap();
//     let _: () = redis_conn.expire(&redis_key, 600).await.unwrap(); // * expire after 10 min

//     println!("🚀 Dev server for {} started at port {}", site_name, port);
// }

// use std::net::TcpListener;

// use dotenvy::dotenv;

// use crate::redis::redis::get_redis_connection;
// use crate::runners::models::DevServerInfo;

// // * Returns a port that is currently free to use
// pub fn get_available_port() -> std::io::Result<u16> {
//     // * Bind to port 0 to let the OS assign a free port
//     let listener = TcpListener::bind("127.0.0.1:0")?;

//     let port = listener.local_addr()?.port();
//     // Drop the listener immediately to free it
//     drop(listener);
//     Ok(port)
// }
