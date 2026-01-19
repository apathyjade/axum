use axum::{
    Json, Router,
    extract::{Query, State},
    routing::{get, post},
};
use chrono::Utc;
use diesel::{
    ExpressionMethods, RunQueryDsl,
    query_dsl::methods::{FilterDsl, LimitDsl, OffsetDsl, SelectDsl},
};
use serde::Deserialize;
use validator::Validate;

use crate::model::api_response::ApiResponse;
use crate::{
    AppStateArc,
    model::{
        pager::Pager,
        user::{ListQueryParams, NewUser, ViewUser},
    },
    schema, utils,
};

#[derive(Deserialize, Validate)]
struct Params {
    #[validate(required(message = "ID不能为空"))]
    id: Option<i64>,
}
async fn get_list(
    State(app_state): State<AppStateArc>,
    Query(params): Query<ListQueryParams>,
) -> Json<ApiResponse<Vec<ViewUser>>> {
    let pool = app_state.db_pool.clone();
    let conn = &mut *pool.get().unwrap();

    let result = tokio::task::block_in_place(|| {
        let table = schema::users::table;
        let pager = params.pager.unwrap_or(Pager {
            page: Some(1),
            page_size: Some(10),
        });
        table
            .select(schema::users::all_columns)
            .offset(pager.get_offset())
            .limit(pager.get_limit())
            .load::<ViewUser>(conn)
    });
    if let Ok(view_user) = result {
        Json(ApiResponse::success(view_user))
    } else {
        Json(ApiResponse::error("列表查询失败"))
    }
}
async fn get_user<'a>(
    State(app_state): State<AppStateArc>,
    Query(params): Query<Params>,
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
        let table = schema::users::table;
        table
            .filter(schema::users::id.eq(id))
            .select(schema::users::all_columns)
            .get_result(conn)
    });
    if let Ok(view_user) = result {
        Json(ApiResponse::success(view_user))
    } else {
        Json(ApiResponse::error(
            result.err().unwrap().to_string().as_str(),
        ))
    }
}

async fn create_user<'a>(State(app_state): State<AppStateArc>) -> Json<ApiResponse<NewUser<'a>>> {
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
        diesel::insert_into(schema::users::table)
            .values(&new_user)
            .execute(conn)
    });
    if let Ok(_) = result {
        Json(ApiResponse::success(new_user))
    } else {
        Json(ApiResponse::error("服务异常，请稍后再试~~~"))
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
        .route("/list", get(get_list))
        .route("/get", get(get_user))
        .route("/post", post(create_user))
    // .route("/put", put(update_user))    // PUT /users/:id
    // .layer(user_middleware)             // 应用模块专属中间件
}
