use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine as _};
use blake3;
use chrono::{Duration, Utc};
use rand::rngs::OsRng;
use rand::TryRngCore;
use sea_orm::prelude::DateTimeWithTimeZone;

#[derive(Debug, Clone)]
pub struct RefreshMaterial {
    pub token_plain: String, // 仅写 Cookie
    pub token_hash: String,  // 落库
    pub iat: DateTimeWithTimeZone,
    pub exp: DateTimeWithTimeZone,
}

pub fn gen_refresh(ttl_seconds: i64) -> RefreshMaterial {
    let mut buf = [0u8; 48];
    let _ = OsRng.try_fill_bytes(&mut buf);
    let token_plain = URL_SAFE_NO_PAD.encode(buf);
    let token_hash = generate_token_hash(&token_plain);
    let now = Utc::now();
    let iat: DateTimeWithTimeZone = now.into();
    let exp: DateTimeWithTimeZone = (now + Duration::seconds(ttl_seconds)).into();
    RefreshMaterial {
        token_plain,
        token_hash,
        iat: iat,
        exp: exp,
    }
}

pub fn generate_token_hash(token_plain: &str) -> String {
    blake3::hash(token_plain.as_bytes()).to_hex().to_string()
}
