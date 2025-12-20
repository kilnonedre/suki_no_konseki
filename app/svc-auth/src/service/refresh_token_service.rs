use crate::{
    entity::prelude::*,
    r#type::{
        request::refresh_token_req::RefreshTokenPatchReq,
        response::refresh_token_resp::RefreshTokenResp,
    },
};
use actix_web::{http::header, HttpRequest};
use common_type::{AuditStatus, ResponseT};
use common_web::ApiError;
use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};

use crate::{
    entity::refresh_tokens,
    mapper::refresh_token_mapper,
    r#type::request::refresh_token_req::RefreshTokenReq,
    util::refresh_token::{gen_refresh, generate_token_hash},
};

async fn audit_snapshot(
    db: &DatabaseConnection,
    model: &refresh_tokens::Model,
    aud_st: AuditStatus,
) -> Result<(), ApiError> {
    let aud_am = refresh_token_mapper::record_aud(model.clone(), aud_st);

    aud_am.insert(db).await.map_err(|e| {
        log::error!("❌ 插入 Refresh Token 审计失败: {}", e);
        ApiError::Internal
    })?;
    Ok(())
}

pub async fn create(
    db: &DatabaseConnection,
    req: HttpRequest,
    body: &RefreshTokenReq,
    prev_refresh_token: Option<String>,
) -> Result<ResponseT<String>, ApiError> {
    if let Some(prev_token_plain) = prev_refresh_token {
        let revoke_req = RefreshTokenPatchReq {
            prev_id: None,
            is_revoked: Some(true),
        };
        if let Err(e) = patch(db, Some(prev_token_plain), &revoke_req).await {
            log::error!("❌ 撤销旧的 Refresh Token 失败: {:?}", e);
            return Ok(ResponseT::err(
                1,
                5,
                "撤销旧的 Refresh Token 失败".to_string(),
            ));
        }
    }

    let header_ua = req
        .headers()
        .get(header::USER_AGENT)
        .and_then(|v| v.to_str().ok())
        .map(|s| s.to_string());

    let device_id = req
        .headers()
        .get("Device-Id")
        .and_then(|v| v.to_str().ok())
        .map(|s| s.to_string());

    let header_ip = extract_client_ip(&req);

    let mut data = body.clone();

    data.user_agent = header_ua;
    data.ip_address = header_ip;
    data.device_id = device_id;
    data.version = "v1".to_string();

    let refresh_material = gen_refresh(3600);

    data.iat = refresh_material.iat;
    data.exp = refresh_material.exp;
    data.refresh_hash = refresh_material.token_hash;

    let am = refresh_token_mapper::req_to_entity(&data);

    let inserted = match am.insert(db).await {
        Ok(model) => model,
        Err(e) => {
            log::error!("❌ 插入 Refresh Token 失败: {}", e);
            return Ok(ResponseT::err(1, 1, "Refresh Token 插入失败".to_string()));
        }
    };

    if let Err(_) = audit_snapshot(db, &inserted, AuditStatus::Create).await {
        return Ok(ResponseT::err(
            1,
            5,
            "插入 Refresh Token 审计失败".to_string(),
        ));
    }

    Ok(ResponseT::ok(refresh_material.token_plain))
}

pub async fn patch(
    db: &DatabaseConnection,
    prev_refresh_token: Option<String>,
    payload: &RefreshTokenPatchReq,
) -> Result<ResponseT<RefreshTokenResp>, ApiError> {
    let prev_token = prev_refresh_token
        .ok_or_else(|| ApiError::BadRequest("未提供旧的 refresh token".to_string()))?;
    let prev_hash = generate_token_hash(&prev_token);
    let model_opt = match RefreshTokens::find()
        .filter(refresh_tokens::Column::RefreshHash.eq(prev_hash))
        .one(db)
        .await
    {
        Ok(opt) => opt,
        Err(e) => {
            log::error!("db query failed: {e}");
            return Ok(ResponseT::err(1, 2, "数据库查询失败".to_string()));
        }
    };
    let model = match model_opt {
        Some(m) => m,
        None => {
            return Ok(ResponseT::err(
                1,
                3,
                "久的 refresh token 不存在".to_string(),
            ))
        }
    };

    payload.prev_id = model.id

    let am = refresh_token_mapper::patch_req_to_entity(model, payload);
    let saved = match am.update(db).await {
        Ok(opt) => opt,
        Err(e) => {
            log::error!("db query failed: {e}");
            return Ok(ResponseT::err(1, 4, "数据库更新失败".to_string()));
        }
    };

    if let Err(_) = audit_snapshot(db, &saved, AuditStatus::Create).await {
        return Ok(ResponseT::err(
            1,
            5,
            "插入 Refresh Token 审计失败".to_string(),
        ));
    }

    let resp = refresh_token_mapper::entity_to_resp(saved);

    Ok(ResponseT::ok(resp))
}

pub fn extract_client_ip(req: &HttpRequest) -> Option<String> {
    // 优先代理头
    if let Some(forwarded) = req
        .headers()
        .get("X-Forwarded-For")
        .and_then(|v| v.to_str().ok())
        .and_then(|v| v.split(',').next())
        .map(|s| s.trim().to_string())
    {
        return Some(forwarded);
    }

    // 否则使用连接信息
    req.connection_info()
        .realip_remote_addr()
        .map(|s| s.to_string())
}
