// use chrono::{Duration, Utc};
// use jsonwebtoken::{
//     decode, encode, errors::Error as JwtError, DecodingKey, EncodingKey, Header, TokenData,
//     Validation,
// };
// use serde::{Deserialize, Serialize};
// use uuid::Uuid;

// #[derive(Debug, Serialize, Deserialize)]
// pub struct Claims {
//     pub sub: Uuid, // user id
//     pub exp: i64,
//     pub iat: i64,
//     pub role: String, // 可选：角色/权限
// }

// pub fn create_access_token(
//     user_id: Uuid,
//     role: &str,
//     secret: &str,
//     expire_seconds: i64,
// ) -> Result<String, JwtError> {
//     let now = Utc::now();
//     let claims = Claims {
//         sub: user_id,
//         exp: (now + Duration::seconds(expire_seconds)).timestamp(),
//         iat: now.timestamp(),
//         role: role.to_string(),
//     };
//     let token = encode(
//         &Header::default(),
//         &claims,
//         &EncodingKey::from_secret(secret.as_bytes()),
//     )?;
//     Ok(token)
// }

// pub fn verify_access_token(token: &str, secret: &str) -> Result<TokenData<Claims>, JwtError> {
//     decode::<Claims>(
//         token,
//         &DecodingKey::from_secret(secret.as_bytes()),
//         &Validation::default(),
//     )
// }
