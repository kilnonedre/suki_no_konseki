use actix_web::{http::header, web, HttpRequest, HttpResponse, Responder};
use common_type::ResponseT;
use common_web::ApiError;
use sea_orm::DatabaseConnection;

use crate::{
    r#type::{request::login_req::LoginReq, response::login_resp::LoginResp},
    service::auth_service,
};

#[utoipa::path(
    post,
    path = "/api/v1/login",
    tag = "AuthController",
    summary = "用户登录",
    description = r#"
用户登录接口。

此接口会验证用户名与密码的正确性，若验证通过，将返回访问令牌（access token）与刷新令牌（refresh token）。
如果用户名或密码错误，将返回 400 错误。
"#,
    request_body(
        content = LoginReq,
        description = "登录请求体，包含用户名与密码。",
        content_type = "application/json"
    ),
    responses(
        (status = 200, description = "登录成功，返回 access token 与 refresh token。", body = ResponseT<LoginResp>),
        (status = 400, description = "用户名或密码错误，登录失败。"),
        (status = 500, description = "服务器内部错误，验证或生成令牌失败。")
    )
)]
pub async fn login(
    db: web::Data<DatabaseConnection>,
    req: HttpRequest,
    body: web::Json<LoginReq>,
) -> Result<impl Responder, ApiError> {
    let resp = auth_service::login(db.get_ref(), req, &body.0).await?;
    Ok(HttpResponse::Ok().json(resp))
}
