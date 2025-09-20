
mod router;

use dotenv::dotenv;
use std::env;

#[tokio::main]
async fn main() {
    dotenv().ok();
    let host = env::var("HOST")
        .expect("HOST 必须在.env文件或环境变量中设置");
    let port = env::var("PORT")
        .expect("PORT 必须在.env文件或环境变量中设置");
    // build our application with a single route
    let routers = router::all_routes();
    
    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind(format!("{}:{}", host, port))
        .await
        .unwrap();
    axum::serve(listener, routers).await.unwrap();
    println!("listening on http://{}:{}", host, port);
}