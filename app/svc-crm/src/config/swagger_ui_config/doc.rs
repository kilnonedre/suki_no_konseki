use utoipa::OpenApi;

use crate::config::swagger_ui_config::security::BearerSecurity;
use crate::r#type::{
    request::create_user_req::CreateCustomerReq, response::user_resp::CustomerResp,
};

use crate::controller;

#[derive(OpenApi)]
#[openapi(
    paths(
        controller::user_controller::health,
        controller::user_controller::create_customer
    ),
    components(
        schemas(CustomerResp, CreateCustomerReq)
    ),
    tags(
        (name = "crm", description = "CRM APIs")
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
