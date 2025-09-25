use actix_web::{web, HttpRequest, HttpResponse, Responder};
use common_web::ApiError;
use internal_clients::PolicyClient;
use sea_orm::DatabaseConnection;

use crate::{
    r#type::{request::user_req::UserReq, response::user_resp::UserResp},
    service::users_service,
};

#[utoipa::path(
    post,
    path = "/api/v1/users",
    tag = "UsersController",
    summary = "创建客户",
    description = r#"
创建一个新的客户。

此接口会在数据库插入客户基本信息，返回新创建客户的 ID 和名称。
若当前用户无创建权限，将返回 400 错误。
"#,
    request_body(
        content = UserReq,
        description = "新客户的请求体，包含名称、联系方式等基本信息。",
        content_type = "application/json"
    ),
    responses(
        (status = 200, description = "客户创建成功，返回新客户信息。", body = UserResp),
        (status = 400, description = "请求体缺少必填字段或当前用户无权限。"),
        (status = 500, description = "服务器内部错误，插入数据库失败或未知错误。")
    ),
    security(
        ("bearerAuth" = [])
    )
)]
pub async fn create_user(
    db: web::Data<DatabaseConnection>,
    body: web::Json<UserReq>,
    policy: web::Data<PolicyClient>,
    req: HttpRequest,
) -> Result<impl Responder, ApiError> {
    let resp = users_service::create_user(db.get_ref(), &body.0, &policy, &req).await?;
    Ok(HttpResponse::Created().json(resp))
}
