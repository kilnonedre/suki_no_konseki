use crate::entity::{prelude::*, users};
use crate::r#type::request::user_req::UpdateUserReq;
use argon2::password_hash::rand_core::OsRng;
use argon2::password_hash::SaltString;
use argon2::{Argon2, PasswordHasher};
use chrono::Utc;
use common_type::EntityStatus;
use common_type::{AuditStatus, PageInfo};
use common_type::{ResponseListT, ResponseT};
use common_web::ApiError;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, PaginatorTrait, QueryFilter,
    QueryOrder, Set,
};
use uuid::Uuid;

use crate::mapper::user_mapper;
use crate::r#type::{request::user_req::CreateUserReq, response::user_resp::UserResp};

async fn audit_user_snapshot(
    db: &DatabaseConnection,
    model: &users::Model,
    aud_st: AuditStatus,
) -> Result<(), ApiError> {
    let aud_am = user_mapper::record_aud(model.clone(), aud_st);

    aud_am.insert(db).await.map_err(|e| {
        log::error!("❌ 插入用户审计失败: {}", e);
        ApiError::Internal
    })?;
    Ok(())
}

pub async fn create(
    db: &DatabaseConnection,
    body: &CreateUserReq,
) -> Result<ResponseT<UserResp>, ApiError> {
    let salt = SaltString::generate(&mut OsRng);
    let hash = Argon2::default()
        .hash_password(body.password.as_bytes(), &salt)
        .map_err(|e| {
            log::error!("hash password failed: {e}");
            ApiError::Internal
        })?
        .to_string();

    let mut am = user_mapper::create_req_to_entity(body);
    am.password_hash = Set(hash);

    let inserted = match am.insert(db).await {
        Ok(model) => model,
        Err(e) => {
            log::error!("❌ 插入用户失败: {}", e);
            return Ok(ResponseT::err(1, 1, "用户插入失败".to_string()));
        }
    };

    if let Err(_) = audit_user_snapshot(db, &inserted, AuditStatus::Create).await {
        return Ok(ResponseT::err(1, 5, "用户审计插入失败".to_string()));
    }
    let resp = user_mapper::entity_to_resp(inserted);

    Ok(ResponseT::ok(resp))
}

pub async fn read(db: &DatabaseConnection, id: Uuid) -> Result<ResponseT<UserResp>, ApiError> {
    let model_opt = match Users::find_by_id(id).one(db).await {
        Ok(opt) => opt,
        Err(e) => {
            log::error!("db query failed: {e}");
            return Ok(ResponseT::err(1, 2, "数据库查询失败".to_string()));
        }
    };

    let model = match model_opt {
        Some(m) => m,
        None => return Ok(ResponseT::err(1, 3, "用户不存在".to_string())),
    };
    let resp = user_mapper::entity_to_resp(model);

    Ok(ResponseT::ok(resp))
}

pub async fn update(
    db: &DatabaseConnection,
    id: Uuid,
    body: &UpdateUserReq,
) -> Result<ResponseT<UserResp>, ApiError> {
    let model_opt = match Users::find_by_id(id).one(db).await {
        Ok(opt) => opt,
        Err(e) => {
            log::error!("db query failed: {e}");
            return Ok(ResponseT::err(1, 2, "数据库查询失败".to_string()));
        }
    };

    let model = match model_opt {
        Some(m) => m,
        None => return Ok(ResponseT::err(1, 3, "用户不存在".to_string())),
    };
    let am = user_mapper::update_req_to_entity(model, body);
    let saved = match am.update(db).await {
        Ok(opt) => opt,
        Err(e) => {
            log::error!("db query failed: {e}");
            return Ok(ResponseT::err(1, 4, "数据库更新失败".to_string()));
        }
    };

    if let Err(_) = audit_user_snapshot(db, &saved, AuditStatus::Create).await {
        return Ok(ResponseT::err(1, 5, "用户审计插入失败".to_string()));
    }
    let resp = user_mapper::entity_to_resp(saved);

    Ok(ResponseT::ok(resp))
}

pub async fn delete(db: &DatabaseConnection, id: Uuid) -> Result<ResponseT<UserResp>, ApiError> {
    let model_opt = match Users::find_by_id(id).one(db).await {
        Ok(opt) => opt,
        Err(e) => {
            log::error!("db query failed: {e}");
            return Ok(ResponseT::err(1, 2, "数据库查询失败".to_string()));
        }
    };

    let model = match model_opt {
        Some(m) => m,
        None => return Ok(ResponseT::err(1, 3, "用户不存在".to_string())),
    };

    let mut am: users::ActiveModel = model.into();
    am.ent_st = Set(EntityStatus::Deleted);
    am.updated_at = Set(Utc::now().into());
    let saved = match am.update(db).await {
        Ok(opt) => opt,
        Err(e) => {
            log::error!("db query failed: {e}");
            return Ok(ResponseT::err(1, 4, "数据库更新失败".to_string()));
        }
    };

    if let Err(_) = audit_user_snapshot(db, &saved, AuditStatus::Create).await {
        return Ok(ResponseT::err(1, 5, "用户审计插入失败".to_string()));
    }
    let resp = user_mapper::entity_to_resp(saved);

    Ok(ResponseT::ok(resp))
}

pub async fn list(
    db: &DatabaseConnection,
    page: u64,
    size: u64,
) -> Result<ResponseT<ResponseListT<UserResp>>, ApiError> {
    let size = size.max(1);
    let base_query = Users::find()
        .filter(users::Column::EntSt.eq(EntityStatus::Active))
        .order_by_desc(users::Column::CreatedAt);
    let total_elements = match base_query.clone().count(db).await {
        Ok(count) => count,
        Err(e) => {
            log::error!("db query failed: {e}");
            return Ok(ResponseT::err(1, 4, "数据总数获取失败".to_string()));
        }
    };
    let paginator = base_query.paginate(db, size);
    let page_index = page.saturating_sub(1);
    let models = paginator.fetch_page(page_index).await.map_err(|e| {
        log::error!("paginate users failed: {e}");
        ApiError::Internal
    })?;

    let total_pages = ((total_elements + size - 1) / size).max(1);

    let page_info = PageInfo {
        page,
        size,
        total_page: total_pages,
        total_element: total_elements,
    };

    let items = models
        .into_iter()
        .map(user_mapper::entity_to_resp)
        .collect();

    Ok(ResponseT::ok(ResponseListT { items, page_info }))
}
