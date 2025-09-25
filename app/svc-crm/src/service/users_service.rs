use actix_web::HttpRequest;
use argon2::password_hash::rand_core::OsRng;
use argon2::password_hash::SaltString;
use argon2::{Argon2, PasswordHasher};
use common_type::entity_status::EntityStatus;
use common_type::ResponseT;
use common_web::ApiError;
use internal_clients::PolicyClient;
use sea_orm::{ActiveModelTrait, DatabaseConnection, EntityTrait, Set};
use uuid::Uuid;

use crate::entity::prelude::*;
use crate::entity::users;
use crate::r#type::{request::user_req::UserReq, response::user_resp::UserResp};

pub async fn create_user(
    db: &DatabaseConnection,
    body: &UserReq,
    policy: &PolicyClient,
    req: &HttpRequest,
) -> Result<ResponseT<UserResp>, ApiError> {
    let salt = SaltString::generate(&mut OsRng);
    let hash = Argon2::default()
        .hash_password(body.password.as_bytes(), &salt)
        .map_err(|e| {
            log::error!("hash password failed: {e}");
            ApiError::Internal
        })?
        .to_string();

    let am = users::ActiveModel {
        username: Set(body.username.to_string()),
        email: Set(body.email.clone()),
        phone: Set(body.phone.clone()),
        password_hash: Set(hash),
        display_name: Set(body.display_name.clone()),
        role: Set("admin".to_string()),
        ent_st: Set(EntityStatus::Active),
        ..Default::default()
    };

    let inserted = am.insert(db).await.map_err(|e| {
        log::error!("❌ 插入用户失败: {}", e);
        ApiError::Internal
    })?;

    let resp = UserResp {
        id: inserted.id,
        ent_st: EntityStatus::Active,
        username: inserted.username,
        email: inserted.email,
        phone: inserted.phone,
        display_name: inserted.display_name,
        role: inserted.role,
        created_at: inserted.created_at,
        updated_at: inserted.updated_at,
        created_by: inserted.created_by,
        updated_by: inserted.updated_by,
        updated_reason: inserted.updated_reason,
    };

    Ok(ResponseT::ok(resp))
}
