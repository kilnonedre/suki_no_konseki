use crate::controller;
use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    paths(
        controller::auth_controller::login,
    ),
    components(
        schemas()
    ),
    tags(
        (name = "AuthController", description = "登录接口面板")
    ),
    // modifiers(&BearerSecurity),
    // security(
    //     ("bearerAuth" = []),
    // )
)]
pub struct ApiDoc;

/// 暴露给 main 使用：构建 OpenAPI
pub fn build_openapi() -> utoipa::openapi::OpenApi {
    ApiDoc::openapi()
}
