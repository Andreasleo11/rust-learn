use jsonwebtoken::errors::Result as JWTResult;
use jsonwebtoken::{Algorithm, DecodingKey, EncodingKey, Header, Validation, decode, encode};
use serde::{Deserialize, Serialize};
use std::env;

#[derive(Serialize, Deserialize)]
pub struct Claims {
    pub sub: String, // subject (user_id)
    pub exp: usize,  // expiration timestamp
}

pub fn create_jwt(user_id: &str) -> JWTResult<String> {
    let expiration = chrono::Utc::now()
        .checked_add_signed(chrono::Duration::minutes(60)) // expired in 60 minutes
        .expect("valid timestamp")
        .timestamp() as usize;

    let claims = Claims {
        sub: user_id.to_string(),
        exp: expiration,
    };

    let header = Header::new(Algorithm::HS256);
    let secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set");

    encode(
        &header,
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )
}

pub fn validate_jwt(token: &str) -> JWTResult<Claims> {
    let secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set");
    decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &Validation::default(),
    )
    .map(|data| data.claims)
}
