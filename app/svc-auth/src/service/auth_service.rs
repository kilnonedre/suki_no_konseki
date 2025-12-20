use std::str::FromStr;

use actix_web::HttpRequest;
use common_grpc::LoginGrpcClient;
use common_type::ResponseT;
use common_web::ApiError;
use sea_orm::DatabaseConnection;
use uuid::Uuid;

use crate::{
    r#type::{
        request::{login_req::LoginReq, refresh_token_req::RefreshTokenReq},
        response::login_resp::LoginResp,
    },
    service::refresh_token_service,
    util::jwt::create_access_token,
};

pub async fn login(
    db: &DatabaseConnection,
    req: HttpRequest,
    body: &LoginReq,
) -> Result<ResponseT<LoginResp>, ApiError> {
    let id: String = match LoginGrpcClient::verify(&body.account, &body.password).await {
        Ok(id) => id,
        Err(e) => {
            log::error!("❌ token 生成失败: {}", e);
            return Ok(ResponseT::err(1, 1, "access_token 生成失败".to_string()));
        }
    };
    println!("gRPC 返回结果: {:?}", id);
    let user_uuid = match Uuid::from_str(&id) {
        Ok(u) => u,
        Err(e) => {
            log::error!("❌ invalid uuid from grpc response: {}", e);
            return Ok(ResponseT::err(1, 1, "无效的用户 ID".to_string()));
        }
    };
    let access_token = match create_access_token(user_uuid, "role", "secret", 3600, "v1") {
        Ok(token) => token,
        Err(e) => {
            log::error!("❌ token 生成失败: {}", e);
            return Ok(ResponseT::err(1, 1, "access_token 生成失败".to_string()));
        }
    };

    let refresh_token_req = RefreshTokenReq {
        user_id: user_uuid,
        created_by: user_uuid,
        ..Default::default()
    };

    let refresh_token = match refresh_token_service::create(db, req, &refresh_token_req, None).await
    {
        Ok(model) => model,
        Err(e) => {
            log::error!("❌ 插入 Refresh Token 失败: {}", e);
            return Ok(ResponseT::err(1, 1, "Refresh Token 插入失败".to_string()));
        }
    };

    let refresh_token_data = refresh_token.data.ok_or_else(|| {
        log::error!("❌ refresh_token.data 为空");
        ApiError::Internal
    })?;

    let resp = LoginResp {
        access_token,
        refresh_token: refresh_token_data,
    };

    Ok(ResponseT::ok(resp))
}
