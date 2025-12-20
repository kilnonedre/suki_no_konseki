use chrono::Utc;
use jsonwebtoken::{
    decode, encode, errors::Error as JwtError, Algorithm, DecodingKey, EncodingKey, Header,
    TokenData, Validation,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    pub sub: Uuid, // user id
    pub exp: i64,
    pub iat: i64,
    pub role: String, // 可选：角色/权限
    pub ver: String,
}

pub fn create_access_token(
    user_id: Uuid,
    role: &str,
    secret: &str,
    expire_seconds: i64,
    version: &str,
) -> Result<String, JwtError> {
    let now = Utc::now().timestamp();
    let claims = Claims {
        sub: user_id,
        exp: now + expire_seconds,
        iat: now,
        role: role.to_string(),
        ver: version.to_string(),
    };
    let header = Header::new(Algorithm::HS256);
    encode(
        &header,
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )
}

pub fn verify_access_token(token: &str, secret: &str) -> Result<TokenData<Claims>, JwtError> {
    let mut v = Validation::new(Algorithm::HS256);
    v.leeway = 30;
    v.validate_exp = true;
    decode::<Claims>(token, &DecodingKey::from_secret(secret.as_bytes()), &v)
}
