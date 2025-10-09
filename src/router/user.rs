
use axum::{
    extract::{ State, Query },
    routing::{get, post},
    Json, Router
};
use diesel::{
    query_dsl::methods::FilterDsl,
    query_dsl::methods::SelectDsl,
    ExpressionMethods,
    RunQueryDsl,
};
use chrono::{ Utc };
use serde::Deserialize;
use validator::Validate;

use crate::{model::user::{NewUser, ViewUser}, schema, utils, AppStateArc};
use crate::model::api_response::ApiResponse;

#[derive(Deserialize, Validate)]
struct Params {
    #[validate(required(message = "ID不能为空"))]
    id: Option<i64>,
}

async fn get_user<'a>(
    State(app_state): State<AppStateArc>,
    Query(params): Query<Params>
) -> Json<ApiResponse<ViewUser>> {
    if let Err(err) = params.validate() {
        let msg = utils::validator::get_validator_first_error_message(&err);
        return Json(ApiResponse::error(&msg));
    }
    let id = params.id.unwrap();
    let pool = app_state.db_pool.clone();
    let conn = &mut *pool.get().unwrap();
    // 执行数据库操作
    let result = tokio::task::block_in_place(|| {
        let table = schema::user::table;
        table.filter(schema::user::id.eq(id))
            .select(schema::user::all_columns)
            .get_result(conn)
    });
    if let Ok(view_user) = result {
        return Json(ApiResponse::success(view_user));
    } else {
        return Json(ApiResponse::error(result.err().unwrap().to_string().as_str()));
        // return Json(ApiResponse::error("服务异常，请稍后再试~~~"));
    }
}

async fn create_user<'a>(State(app_state)
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
    let conn = &mut *pool.get().unwrap();
    // 执行数据库操作
    let result = tokio::task::block_in_place(|| {
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
        .route("/post", post(create_user))
        // .route("/put", put(update_user))    // PUT /users/:id
        // .layer(user_middleware)             // 应用模块专属中间件
}
