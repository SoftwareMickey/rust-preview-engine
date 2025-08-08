#![allow(non_snake_case)]

use std::io::Result;

pub async fn httpCreateViteConfig(project_path : &str, _project_name : &str, 
    user_id : &str, site_name : &str) -> Result<()>{

    // let project_dir = format!("{}/{}", project_path, project_name);

    let basePath = format!("/sites/{}/{}/", user_id, site_name);

    let vite_path = format!("{}/vite.config.js", project_path);
    let viteLayoutConfig = format!(r#"
import {{ defineConfig }} from 'vite';
import react from '@vitejs/plugin-react';
import tailwindcss from '@tailwindcss/vite'

export default defineConfig({{
    base: '{}',
    server : {{
        port : 5174
    }},
    plugins: [react(), tailwindcss()],
}});
"#, basePath).trim_start().to_string();

    std::fs::write(vite_path, viteLayoutConfig).expect("FAILED TO WRITE TO VITE PATH");

    Ok(())
}