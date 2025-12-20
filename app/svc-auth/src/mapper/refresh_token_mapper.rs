use chrono::{FixedOffset, Utc};
use common_type::{AuditStatus, EntityStatus};
use sea_orm::{ActiveValue::Set, IntoActiveModel};

use crate::{
    entity::{refresh_tokens, refresh_tokens_aud},
    r#type::{
        request::refresh_token_req::{RefreshTokenPatchReq, RefreshTokenReq},
        response::refresh_token_resp::RefreshTokenResp,
    },
};

pub fn entity_to_resp(entity: refresh_tokens::Model) -> RefreshTokenResp {
    RefreshTokenResp {
        id: entity.id,
        ent_st: entity.ent_st,
        prev_id: entity.prev_id,
        user_id: entity.user_id,
        refresh_hash: entity.refresh_hash,
        ip_address: entity.ip_address,
        user_agent: entity.user_agent,
        exp: entity.exp,
        iat: entity.iat,
        is_revoked: entity.is_revoked,
        created_at: entity.created_at,
        created_by: entity.created_by,
        updated_at: entity.updated_at,
        updated_by: entity.updated_by,
        device_id: entity.device_id,
        version: entity.version,
        updated_reason: entity.updated_reason,
    }
}

pub fn req_to_entity(req: &RefreshTokenReq) -> refresh_tokens::ActiveModel {
    refresh_tokens::ActiveModel {
        ent_st: Set(EntityStatus::Active),
        prev_id: Set(req.prev_id),
        user_id: Set(req.user_id.clone()),
        refresh_hash: Set(req.refresh_hash.clone()),
        ip_address: Set(req.ip_address.clone()),
        user_agent: Set(req.user_agent.clone()),
        exp: Set(req.exp.clone()),
        iat: Set(req.iat.clone()),
        is_revoked: Set(req.is_revoked.clone()),
        created_by: Set(req.created_by.clone()),
        device_id: Set(req.device_id.clone()),
        version: Set(req.version.clone()),
        ..Default::default()
    }
}

pub fn patch_req_to_entity(
    old: refresh_tokens::Model,
    req: &RefreshTokenPatchReq,
) -> refresh_tokens::ActiveModel {
    let mut am = old.into_active_model();
    if let Some(prev_id) = req.prev_id {
        am.prev_id = Set(Some(prev_id));
    }
    if let Some(is_revoked) = req.is_revoked {
        am.is_revoked = Set(is_revoked);
    }
    let offset = FixedOffset::east_opt(0).unwrap();
    am.updated_at = Set(Some(Utc::now().with_timezone(&offset)));
    am
}

// pub fn update_req_to_entity(old: users::Model, req: &UpdateUserReq) -> users::ActiveModel {
//     let mut am = old.into_active_model();
//     am.username = Set(req.base.username.clone());
//     am.email = Set(req.base.email.clone());
//     am.phone = Set(req.base.phone.clone());
//     am.display_name = Set(req.base.display_name.clone());
//     am.role = Set(req.base.role.clone());
//     am.updated_at = Set(Utc::now().into());
//     am
// }

pub fn record_aud(
    model: refresh_tokens::Model,
    aud_st: AuditStatus,
) -> refresh_tokens_aud::ActiveModel {
    refresh_tokens_aud::ActiveModel {
        aud_st: Set(aud_st.clone()),
        ent_id: Set(Some(model.id)),
        ent_st: Set(Some(model.ent_st.clone())),
        prev_id: Set(model.prev_id),
        user_id: Set(Some(model.user_id)),
        refresh_hash: Set(Some(model.refresh_hash)),
        ip_address: Set(model.ip_address),
        user_agent: Set(model.user_agent),
        exp: Set(Some(model.exp)),
        iat: Set(Some(model.iat)),
        is_revoked: Set(Some(model.is_revoked)),
        created_at: Set(Some(model.created_at)),
        created_by: Set(Some(model.created_by)),
        updated_at: Set(model.updated_at),
        updated_by: Set(model.updated_by),
        device_id: Set(model.device_id),
        version: Set(Some(model.version)),
        updated_reason: Set(model.updated_reason),
        ..Default::default()
    }
}
