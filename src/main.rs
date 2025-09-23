mod router;
mod utils;
mod model;
use dotenv::dotenv;
use std::env;
use std::sync::{Arc};

pub use utils::db;

#[derive(Debug, Clone)]
pub struct AppState {
    pub pool: db::DbPool,
}
pub type AppStateArc = Arc<AppState>;

#[tokio::main]
async fn main() {
    dotenv().ok();
    let host = env::var("HOST")
        .expect("HOST 必须在.env文件或环境变量中设置");
    let port = env::var("PORT")
        .expect("PORT 必须在.env文件或环境变量中设置");

    
    let pool = db::establish_connection_pool().await;
    let app_state = AppState { pool };
    let app_state_arc = Arc::new(app_state);

    
    // build our application with a single route
    let routers = router::all_routes().with_state(app_state_arc);
    let listener = tokio::net::TcpListener::bind(format!("{}:{}", host, port))
        .await
        .unwrap();
    axum::serve(listener, routers.into_make_service())
        .await
        .unwrap();
    println!("listening on http://{}:{}", host, port);
}