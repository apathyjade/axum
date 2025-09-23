
use axum::{
    extract::{Query, State},
    routing::get, Json, Router
};
use serde::Deserialize;

use crate::AppStateArc;
use crate::model::api_response::ApiResponse;
use crate::model::user::NewUser;

#[derive(Deserialize)]
struct Params {
    id: i32,
}

async fn get_user<'a>(State(app_state): State<AppStateArc>, Query(params): Query<Params>) -> Json<ApiResponse<NewUser>> {
  let res = sqlx::query_as!(
      NewUser,
      r#"select name, email from "user" where id = $1"#,
      params.id
    )
    .fetch_one(&app_state.pool).await;
  match res {
      Ok(row) => {
          return Json(ApiResponse::success(row));
      },
      Err(_) => {
          return Json(ApiResponse::error("查询用户信息失败"));
      }
  }
}
// async fn create_user<'a>() -> Json<User<'a>> {
//   return Json(User { name: "Hello World" });
// }
// async fn update_user<'a>() -> Json<User<'a>> {
//   return Json(User { name: "Hello World" });
// }
pub fn router<'a>() -> Router<AppStateArc> {
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
        // .route("/post", post(create_user))      // POST /users
        // .route("/put", put(update_user))    // PUT /users/:id
        // .layer(user_middleware)             // 应用模块专属中间件
}
