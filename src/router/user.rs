use axum::{
    routing::{get, post, put},
    Json, Router,
};
use serde::{ Serialize, Deserialize };

#[derive(Serialize, Deserialize)]
struct User<'a> {
    name: &'a str,
}

async fn get_user<'a>() -> Json<User<'a>> {
  return Json(User { name: "Hello World" });
}
async fn create_user<'a>() -> Json<User<'a>> {
  return Json(User { name: "Hello World" });
}
async fn update_user<'a>() -> Json<User<'a>> {
  return Json(User { name: "Hello World" });
}
pub fn router<'a>() -> Router {
    // 模块专属中间件（如用户认证）
    // let user_middleware = tower::layer::LayerFn::new(|service| {
    //     // 模拟认证中间件
    //     tower::service_fn(move |req| {
    //         println!("用户模块中间件：验证令牌");
    //         service.call(req)
    //     })
    // });

    Router::new()
        .route("/get", get(get_user))       // GET /users/:id
        .route("/post", post(create_user))      // POST /users
        .route("/put", put(update_user))    // PUT /users/:id
        // .layer(user_middleware)             // 应用模块专属中间件
}
