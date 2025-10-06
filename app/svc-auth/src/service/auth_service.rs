use std::str::FromStr;

use common_grpc::LoginGrpcClient;
use common_type::ResponseT;
use common_web::ApiError;
use sea_orm::DatabaseConnection;
use uuid::Uuid;

use crate::{
    r#type::{request::login_req::LoginReq, response::login_resp::LoginResp},
    util::jwt::create_access_token,
};

pub async fn login(
    db: &DatabaseConnection,
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
    let access_token = match create_access_token(user_uuid, "role", "secret", 3600) {
        Ok(token) => token,
        Err(e) => {
            log::error!("❌ token 生成失败: {}", e);
            return Ok(ResponseT::err(1, 1, "access_token 生成失败".to_string()));
        }
    };

    let resp = LoginResp { access_token };

    Ok(ResponseT::ok(resp))
}
