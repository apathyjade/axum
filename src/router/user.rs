
use axum::{
    extract::{ State },
    routing::get, Json, Router
};
use diesel::{ RunQueryDsl};
use chrono::{ Utc };

use crate::{model::user::NewUser, utils, schema, AppStateArc};
use crate::model::api_response::ApiResponse;



async fn get_user<'a>(State(app_state)
: State<AppStateArc>)
-> Json<ApiResponse<NewUser<'a>>> {
    let password = utils::password::hash_password("wxy0809").expect("msg");
    let new_user: NewUser<'a> = NewUser {
        username: "test",
        password,
        email: "apathyjade@outlook.com",
        phone: "18632798101",
        real_name: "测试",
        status: 0,
        created_time: Utc::now().naive_utc(),
        updated_time: Utc::now().naive_utc(),
    };
    let pool = app_state.db_pool.clone();
    // 执行数据库操作
    let result = tokio::task::block_in_place(|| {
        let conn = &mut *pool.get().unwrap();
        diesel::insert_into(schema::user::table)
            .values(&new_user)
            .execute(conn)
    });
    if let Ok(_) = result {
        return Json(ApiResponse::success(new_user));
    } else {
        return Json(ApiResponse::error("服务异常，请稍后再试~~~"));
    }
}
// #[derive(Deserialize)]
// struct Params {
//     id: i64,
// }
// async fn create_user<'a>(Query(params): Query<Params>) -> Json<User<'a>> {
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
