use chrono::Utc;
use common_type::{AuditStatus, EntityStatus};
use sea_orm::{ActiveValue::Set, IntoActiveModel};

use crate::{
    entity::{users, users_aud},
    r#type::{
        request::user_req::{CreateUserReq, UpdateUserReq},
        response::user_resp::UserResp,
    },
};

pub fn entity_to_resp(entity: users::Model) -> UserResp {
    UserResp {
        id: entity.id,
        ent_st: entity.ent_st,
        username: entity.username,
        email: entity.email,
        phone: entity.phone,
        display_name: entity.display_name,
        role: entity.role,
        created_at: entity.created_at,
        updated_at: entity.updated_at,
        created_by: entity.created_by,
        updated_by: entity.updated_by,
        updated_reason: entity.updated_reason,
    }
}

pub fn create_req_to_entity(req: &CreateUserReq) -> users::ActiveModel {
    users::ActiveModel {
        ent_st: Set(EntityStatus::Active),
        username: Set(req.base.username.clone()),
        email: Set(req.base.email.clone()),
        phone: Set(req.base.phone.clone()),
        display_name: Set(req.base.display_name.clone()),
        role: Set(req.base.role.clone()),
        ..Default::default()
    }
}

pub fn update_req_to_entity(old: users::Model, req: &UpdateUserReq) -> users::ActiveModel {
    let mut am = old.into_active_model();
    am.username = Set(req.base.username.clone());
    am.email = Set(req.base.email.clone());
    am.phone = Set(req.base.phone.clone());
    am.display_name = Set(req.base.display_name.clone());
    am.role = Set(req.base.role.clone());
    am.updated_at = Set(Utc::now().into());
    am
}

pub fn record_aud(model: users::Model, aud_st: AuditStatus) -> users_aud::ActiveModel {
    users_aud::ActiveModel {
        aud_st: Set(aud_st.clone()),
        ent_id: Set(Some(model.id)),
        ent_st: Set(Some(model.ent_st.clone())),
        username: Set(Some(model.username.clone())),
        email: Set(model.email.clone()),
        phone: Set(model.phone.clone()),
        password_hash: Set(Some(model.password_hash.clone())),
        display_name: Set(model.display_name.clone()),
        role: Set(Some(model.role.clone())),
        created_at: Set(Some(model.created_at.clone())),
        updated_at: Set(Some(model.updated_at.clone())),
        created_by: Set(model.created_by.clone()),
        updated_by: Set(model.updated_by.clone()),
        updated_reason: Set(model.updated_reason.clone()),
        ..Default::default()
    }
}
