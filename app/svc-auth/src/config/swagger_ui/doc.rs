use common_config::BearerSecurity;
use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    paths(
    ),
    components(
        schemas()
    ),
    tags(
        (name = "UsersController", description = "用户接口面板")
    ),
    modifiers(&BearerSecurity),
    security(
        ("bearerAuth" = []),
    )
)]
pub struct ApiDoc;

/// 暴露给 main 使用：构建 OpenAPI
pub fn build_openapi() -> utoipa::openapi::OpenApi {
    ApiDoc::openapi()
}
