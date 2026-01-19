use crate::model::api_response::ApiResponse;
use crate::model::tenant::{ListQueryParams, Tenant};
use crate::service::tenant as service_tenant;
use crate::{AppStateArc, model::tenant::NewTenant};
use axum::{
    Router,
    extract::{Json, Query, State},
    routing::{get, post},
};
use utoipa;

#[utoipa::path(
    get,
    path = "/api/tenant/list",
    params(
        ("id" = i32, Path, description = "用户ID", example = 1)
    ),
    responses(
        (status = 200, description = "成功返回用户列表", body = ApiResponse<Vec<Tenant>>)
    ),
    tag = "tenant"
)]
pub async fn get_list(
    State(app_state): State<AppStateArc>,
    Query(params): Query<ListQueryParams>,
) -> Json<ApiResponse<Vec<Tenant>>> {
    let mut conn = app_state.db_pool.get().unwrap();
    let result = service_tenant::get_list(&mut conn, params.pager);
    return match result {
        Ok(tenants) => Json(ApiResponse::success(tenants)),
        Err(err) => Json(ApiResponse::error(err.as_str())),
    };
}

pub async fn create_tenant(
    State(app_state): State<AppStateArc>,
    Json(new_tenant): Json<NewTenant>,
) -> Json<ApiResponse<Tenant>> {
    let mut conn = app_state.db_pool.get().unwrap();
    let result = service_tenant::create_tenant(&mut conn, new_tenant).await;
    match result {
        Ok(tenant) => Json(ApiResponse::success(tenant)),
        Err(err) => Json(ApiResponse::error(err.as_str())),
    }
}
pub fn router<'a>() -> Router<AppStateArc> {
    Router::new()
        .route("/list", get(get_list))
        .route("/create", post(create_tenant))
}
