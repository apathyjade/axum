
// use serde_yaml;
// use std::path::Path;
// use tokio::fs;

// include!("../api_doc.rs");
// // use crate::ApiDoc;

// #[tokio::main]
// async fn main() {
//     let output_dir = Path::new("./opneapi");
//     if !output_dir.exists() {
//         fs::create_dir(output_dir).await.unwrap();
//     }
//     let spec = ApiDoc::openapi();
//     let yaml = serde_yaml::to_string(&spec).unwrap();
//     fs::write("./opneapi/openapi.yaml", yaml).await.unwrap();
//     println!("cargo:warning=Generated openapi.yaml");
// }
