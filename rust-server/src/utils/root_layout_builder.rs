#![allow(non_snake_case)]
use std::io::Result;

pub async fn httpCreateRootLayout(project_path : &str, _project_name : &str) -> Result<()>{

    // let project_dir = format!("{}/{}", project_path, project_name);

    let routes_path = format!("{}/src/RootLayout.jsx", project_path);
    let rootLayoutConfig = r#"
import { Outlet } from "react-router";

export default function RootLayout(){
    return <>
        <Outlet/>
    </>
}
"#.trim_start().to_string();

    std::fs::write(routes_path, rootLayoutConfig).expect("FAILED TO WRITE TO CREATE LAYOUT FILE...");

    Ok(())
}

pub async fn httpCreateRoutesLayout(
  project_path : &str, 
  _project_name : &str, user_id : &str, site_name : &str) -> Result<()>{

    // let project_dir = format!("{}/{}", project_path, project_name);

    let routes_path = format!("{}/src/Routes.jsx", project_path);
    let routesLayoutConfig = format!(r#"
import {{ createBrowserRouter, RouterProvider }} from 'react-router-dom'
import RootLayout from './RootLayout'

export default function Routes() {{
  const routes = createBrowserRouter([
    {{
      path: "/sites/{}/{}",
      element: <RootLayout />,
      children: [
        {{ index: true, path: '', element: '' }},
      ]
  }},
  ]);

  return <div>
    <RouterProvider router={{routes}}/>
  </div>;
  }}
"#, user_id, site_name).trim_start().to_string();

    std::fs::write(routes_path, routesLayoutConfig).expect("FAILED TO WRITE ROUTES FILE");

    Ok(())
}


pub async fn httpMergeLayoutIntoApp(project_path : &str, _project_name : &str) -> Result<()>{

    // let project_dir = format!("{}/{}", project_path, project_name);

    let app_path = format!("{}/src/App.jsx", project_path);
    let appLayoutConfig = r#"
import Routes from './Routes';

function App() {
  return <Routes/>;
}

export default App;
"#.trim_start().to_string();

    std::fs::write(app_path, appLayoutConfig).expect("FAILED TO WRITE TO MERGE...");

    Ok(())
}