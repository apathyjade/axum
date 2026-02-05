use utoipa::OpenApi;
use crate::model::{self, tenant};
use crate::router::{tenant as r_tenant};

// 在 docs.rs 中聚合
#[derive(OpenApi)]
#[openapi(
    info(
        title = "API", 
        description = "API 文档",
        terms_of_service = "www.baidu.com",
    ),
    servers(
        (url = "http://localhost:3000", description = "desploy server"),
        (url = "https://api.example.com", description = "Production server")
    ),
    paths(
        r_tenant::get_list,
        r_tenant::create_tenant,
    ),
    components(
        schemas(
            model::pager::Pager,
            tenant::Tenant,
            tenant::NewTenant
        )
    ),
    tags(
        (name = "tenant", description = "租户管理"),
        (name = "use", description = "用户管理")
    )
)]
pub struct ApiDoc;
