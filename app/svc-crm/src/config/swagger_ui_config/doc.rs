use utoipa::OpenApi;

use crate::config::swagger_ui_config::security::BearerSecurity;
use crate::r#type::request::user_req::UpdateUserReq;
use crate::r#type::{request::user_req::CreateUserReq, response::user_resp::UserResp};

use crate::controller;

#[derive(OpenApi)]
#[openapi(
    paths(
        controller::user_controller::create,
        controller::user_controller::read,
        controller::user_controller::update,
        controller::user_controller::delete,
        controller::user_controller::list,
    ),
    components(
        schemas(UserResp, CreateUserReq, UpdateUserReq)
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
