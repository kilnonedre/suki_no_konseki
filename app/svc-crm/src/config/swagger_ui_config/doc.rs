use utoipa::OpenApi;

use crate::config::swagger_ui_config::security::BearerSecurity;
use crate::r#type::{request::user_req::UserReq, response::user_resp::UserResp};

use crate::controller;

#[derive(OpenApi)]
#[openapi(
    paths(
        controller::user_controller::create_user
    ),
    components(
        schemas(UserResp, UserReq)
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
