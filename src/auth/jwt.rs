use anyhow::Result;
use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, TokenData, Validation};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub user_id: Uuid,
    pub exp: i64,
    pub iat: i64,
}

impl Claims {
    pub fn new(user_id: Uuid) -> Self {
        let now = Utc::now();
        let exp = now + Duration::hours(24);

        Self {
            sub: user_id.to_string(),
            user_id,
            exp: exp.timestamp(),
            iat: now.timestamp(),
        }
    }
}

pub fn create_jwt(user_id: Uuid, secret: &str) -> Result<String> {
    let claims = Claims::new(user_id);
    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_ref()),
    )?;
    Ok(token)
}

pub fn verify_jwt(token: &str, secret: &str) -> Result<TokenData<Claims>> {
    let token_data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_ref()),
        &Validation::default(),
    )?;
    Ok(token_data)
}