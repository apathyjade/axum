use utoipa::OpenApi;

use crate::router::tenant::get_list as get_tenant_list;

// 在 docs.rs 中聚合
#[derive(OpenApi)]
#[openapi(
    paths(
        get_tenant_list,
    ),
    components(schemas()),
    tags(
        (name = "tenant", description = "zh-CN: 租户管理 en-US: Tenant management")
    )
)]
pub struct ApiDoc;
