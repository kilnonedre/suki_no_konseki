use actix_web::{web, HttpResponse, Responder};
use common_type::{page_req::PageReq, ResponseListT, ResponseT};
use common_web::ApiError;
use sea_orm::DatabaseConnection;

use crate::{
    r#type::{
        request::user_req::{CreateUserReq, UpdateUserReq},
        response::user_resp::UserResp,
    },
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
        content = CreateUserReq,
        description = "新客户的请求体，包含名称、联系方式等基本信息。",
        content_type = "application/json"
    ),
    responses(
        (status = 200, description = "客户创建成功，返回新客户信息。", body = ResponseT<UserResp>),
        (status = 400, description = "请求体缺少必填字段或当前用户无权限。"),
        (status = 500, description = "服务器内部错误，插入数据库失败或未知错误。")
    ),
    security(
        ("bearerAuth" = [])
    )
)]
pub async fn create(
    db: web::Data<DatabaseConnection>,
    body: web::Json<CreateUserReq>,
    // req: HttpRequest,
) -> Result<impl Responder, ApiError> {
    let resp = users_service::create(db.get_ref(), &body.0).await?;
    Ok(HttpResponse::Created().json(resp))
}

#[utoipa::path(
    get,
    path = "/api/v1/users/{id}",
    tag = "UsersController",
    summary = "获取用户详情",
    description = r#"
根据用户 ID 获取用户详情。

返回对应用户的基本信息及扩展资料。
若用户不存在，返回 404。
"#,
    params(
        ("id" = String, Path, description = "用户的唯一标识 UUID"),
    ),
    responses(
        (status = 200, description = "查询成功，返回用户信息。", body = ResponseT<UserResp>),
        (status = 404, description = "指定 ID 的用户不存在。"),
        (status = 500, description = "服务器内部错误，查询失败或未知错误。")
    ),
    security(
        ("bearerAuth" = [])
    )
)]
pub async fn read(
    db: web::Data<DatabaseConnection>,
    path: web::Path<uuid::Uuid>,
    // req: HttpRequest,
) -> Result<impl Responder, ApiError> {
    let user_id = path.into_inner();
    let resp = users_service::read(db.get_ref(), user_id).await?;
    Ok(HttpResponse::Ok().json(resp))
}

#[utoipa::path(
    put,
    path = "/api/v1/users/{id}",
    tag = "UsersController",
    summary = "全量更新用户（不含密码）",
    description = r#"
按 ID 覆盖更新用户的全部可写字段（不允许修改密码）。
请求体中所有字段将覆盖原值；不想要的可传 null（如 email/phone/display_name）。
"#,
    params(
        ("id" = String, Path, description = "用户的唯一标识 UUID"),
    ),
    request_body(
        content = UpdateUserReq,
        description = "全量更新的请求体（不含密码）。",
        content_type = "application/json"
    ),
    responses(
        (status = 200, description = "更新成功，返回用户信息。", body = ResponseT<UserResp>),
        (status = 404, description = "指定 ID 的用户不存在。"),
        (status = 409, description = "唯一约束冲突（如用户名或邮箱已存在）。"),
        (status = 500, description = "服务器内部错误，更新失败或未知错误。")
    ),
    security(
        ("bearerAuth" = [])
    )
)]
pub async fn update(
    db: web::Data<DatabaseConnection>,
    path: web::Path<uuid::Uuid>,
    body: web::Json<UpdateUserReq>,
    // req: HttpRequest,
) -> Result<impl Responder, ApiError> {
    let user_id = path.into_inner();
    let resp = users_service::update(db.get_ref(), user_id, &body.0).await?;
    Ok(HttpResponse::Ok().json(resp))
}

#[utoipa::path(
    delete,
    path = "/api/v1/users/{id}",
    tag = "UsersController",
    summary = "删除用户（软删除）",
    description = r#"
按 ID 删除指定用户。  
此操作为**软删除**：不会物理删除数据库记录，而是将用户状态(`ent_st`)更新为 `DELETED`，以便后续审计或恢复。  
如果用户已是 `DELETED`，操作将幂等执行，返回当前记录。
"#,
    params(
        ("id" = String, Path, description = "用户的唯一标识 UUID"),
    ),
    responses(
        (status = 200, description = "删除成功，返回删除后的用户信息。", body = ResponseT<UserResp>),
        (status = 404, description = "指定 ID 的用户不存在。"),
        (status = 500, description = "服务器内部错误，删除失败或未知错误。")
    ),
    security(
        ("bearerAuth" = [])
    )
)]
pub async fn delete(
    db: web::Data<DatabaseConnection>,
    path: web::Path<uuid::Uuid>,
) -> Result<impl Responder, ApiError> {
    let user_id = path.into_inner();
    let resp = users_service::delete(db.get_ref(), user_id).await?;
    Ok(HttpResponse::Ok().json(resp))
}

#[utoipa::path(
    get,
    path = "/api/v1/users",
    tag = "UsersController",
    summary = "获取用户列表（分页）",
    description = r#"
按条件分页获取用户列表。  
返回的用户均为**活跃状态（`ent_st` = ACTIVE）**。  
支持分页参数 `page`（页码）和 `size`（每页数量）。  
分页信息包含总页数、总条目数等，便于前端显示和导航。
"#,
    params(
        ("page" = Option<u64>, Query, description = "页码，默认 1"),
        ("size" = Option<u64>, Query, description = "每页条目数量，默认 10")
    ),
    responses(
        (status = 200, description = "获取成功，返回用户列表及分页信息。", body = ResponseT<ResponseListT<UserResp>>),
        (status = 500, description = "服务器内部错误，获取失败或未知错误。")
    ),
    security(
        ("bearerAuth" = [])
    )
)]
pub async fn list(
    db: web::Data<DatabaseConnection>,
    query: web::Query<PageReq>,
) -> Result<impl Responder, ApiError> {
    let page = query.page;
    let size = query.size;
    let resp = users_service::list(db.get_ref(), page, size).await?;
    Ok(HttpResponse::Ok().json(resp))
}
