mod macros;
mod middleware;
mod model;
mod router;
mod schema;
mod service;
mod utils;

use dotenv::dotenv;
use std::sync::Arc;
use utils::db;

use model::api_res::{ ApiRes };
use model::api_response::{ AppErr };

use model::app_state::{AppState, AppStateArc};

#[tokio::main(flavor = "multi_thread", worker_threads = 4)]
async fn main() {
    dotenv().ok();
    let host = utils::env::get_env(utils::env::Env::Host);
    let port = utils::env::get_env(utils::env::Env::Port);

    let db_pool = db::init_diesel_db().await;
    let gis_db_pool = db::init_gis_db().await;
    let app_state = AppState { db_pool, gis_db_pool };
    let app_state_arc = Arc::new(app_state);


    // build our application with a single route
    let routers = router::all_routes().with_state(app_state_arc);

    let listener = tokio::net::TcpListener::bind(format!("{}:{}", host, port))
        .await
        .unwrap();

    println!("Server running on http://{}:{}", host, port);

    axum::serve(listener, routers.into_make_service())
        .await
        .unwrap();
}
