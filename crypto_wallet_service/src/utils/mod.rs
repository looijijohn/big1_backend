use jsonwebtoken::{encode, Header, EncodingKey, DecodingKey, Validation, errors::Error as JwtError};
use chrono::{Utc, Duration};
use crate::models::Claims;
use crate::errors::AppError;

pub fn create_jwt(user_id: &str, secret: &str) -> Result<String, JwtError> {
    let expiration = Utc::now() + Duration::hours(24);
    let claims = Claims {
        sub: user_id.to_string(),
        exp: expiration.timestamp(),
    };
    encode(&Header::default(), &claims, &EncodingKey::from_secret(secret.as_ref()))
}

pub fn decode_jwt(token: &str, secret: &str) -> Result<Claims, AppError> {
    jsonwebtoken::decode::<Claims>(token, &DecodingKey::from_secret(secret.as_ref()), &Validation::default())
        .map(|data| data.claims)
        .map_err(AppError::from)
}