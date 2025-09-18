use actix_web::HttpRequest;
use common_web::ApiError;
use internal_clients::PolicyClient;
use sea_orm::{ActiveModelTrait, DatabaseConnection, EntityTrait, Set};
use uuid::Uuid;

use crate::entity::prelude::*;
use crate::entity::users;
use crate::r#type::{
    request::create_user_req::CreateCustomerReq, response::user_resp::CustomerResp,
};

pub async fn create_customer(
    db: &DatabaseConnection,
    body: &CreateCustomerReq,
    policy: &PolicyClient,
    req: &HttpRequest,
) -> Result<CustomerResp, ApiError> {
    // ===== 可选：策略校验 =====
    // let user_id = req.headers().get("X-User-Id")
    //     .and_then(|h| h.to_str().ok()).unwrap_or("anonymous");
    // let eval = policy.evaluate(&EvaluateReq{
    //     user_id: Cow::Borrowed(user_id),
    //     resource_type: Cow::Borrowed("customer"),
    //     resource_id: Cow::Borrowed("new"),
    //     action: Cow::Borrowed("create"),
    // }).await.map_err(|_| ApiError::Internal)?;
    // if !eval.allow { return Err(ApiError::BadRequest("not allowed".into())); }

    let am = users::ActiveModel {
        username: Set("demo111".to_string()),
        email: Set(Some("demo@example11.com".to_string())),
        phone: Set(Some("+86-13800000000".to_string())),
        password_hash: Set("$argon2id$v=19$m=4096,t=3,p=1$xxx".to_string()),
        display_name: Set(Some("演示用户".to_string())),
        role: Set("admin".to_string()),
        ent_st: Set("ACTIVE".to_string()),
        created_by: Set(Some(0)),
        updated_by: Set(Some(0)),
        ..Default::default()
    };

    let _inserted = am.insert(db).await.map_err(|e| {
        log::error!("❌ 插入用户失败: {}", e);
        ApiError::Internal
    })?;

    let user_list = Users::find().all(db).await.map_err(|e| {
        log::error!("query users failed: {e}");
        ApiError::Internal
    })?;

    Ok(CustomerResp {
        id: Uuid::new_v4(),
        name: body.name.clone(),
    })
}
