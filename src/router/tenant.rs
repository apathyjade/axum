
use crate::model::api_response::{ApiResponse, AppErr};
use crate::model::tenant::{ListQueryParams, Tenant};
use crate::service::tenant as service_tenant;
use crate::{AppStateArc, model::tenant::NewTenant};
use axum::{
    Router,
    extract::{Json, Query, State},
    routing
};
use diesel;
use diesel::Connection;
use utoipa;

use crate::ApiRes;

#[utoipa::path(
    get, path = "/api/tenant/list", tag = "tenant",
    params(
        ("id" = i32, Path, description = "用户ID", example = 1)
    ),
    responses(
        (status = 200, description = "成功返回用户列表", body = ApiResponse<Vec<Tenant>>)
    ),
)]
pub async fn get_list(
    State(app_state): State<AppStateArc>,
    Query(params): Query<ListQueryParams>,
) -> ApiRes<Vec<Tenant>> {
    let mut conn = app_state.db_pool.get().unwrap();
    let result = service_tenant::get_list(&mut conn, params.pager);
    result.map(ApiRes::ok).unwrap_or_else(|_| ApiRes::biz_err("获取列表失败".to_string()))
}

#[utoipa::path(
    post, path = "/api/tenant/create", tag = "tenant",
    request_body = NewTenant,
    responses(
        (status = 200, description = "Success", body = ApiResponse<Tenant>),
    ),
)]
pub async fn create_tenant(
    State(app_state): State<AppStateArc>,
    Json(new_tenant): Json<NewTenant>,
) -> ApiRes<Tenant> {
    let mut conn = app_state.db_pool.get().unwrap();

    let result = conn.transaction(|conn| {
        let c_name = new_tenant.company_name.clone();
        let exist: bool = service_tenant::is_tenant_exists(conn, c_name)?;
        if exist {
            return Err(AppErr::DatabaseError("租户已存在！".to_string()));
        }
        service_tenant::create_tenant(conn, new_tenant).map_err(|_| {
            AppErr::DatabaseError("创建租户失败！".to_string())
        })
    });
    result.map(ApiRes::ok).unwrap_or_else(|err| err.into())
}

pub fn router<'a>() -> Router<AppStateArc> {
    Router::new()
        .route("/list", routing::get(get_list))
        .route("/create", routing::post(create_tenant))
}
