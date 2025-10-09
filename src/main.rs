mod router;
mod utils;
mod model;
mod schema;
mod middleware;
use dotenv::dotenv;
use std::env;
use std::sync::{Arc};

pub use utils::db;

#[derive(Debug, Clone)]
pub struct AppState {
    pub db_pool: db::DbPool,
}
pub type AppStateArc = Arc<AppState>;

#[tokio::main(flavor = "multi_thread", worker_threads = 1)]
async fn main() {
    dotenv().ok();
    let host = env::var("HOST")
        .expect("HOST 必须在.env文件或环境变量中设置");
    let port = env::var("PORT")
        .expect("PORT 必须在.env文件或环境变量中设置");

    let db_pool = db::init_diesel_db().await;
    let app_state = AppState { db_pool };
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