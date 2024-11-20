// src/models/user.rs
use serde::{Serialize, Deserialize};
use chrono::{Utc, Duration};
use jsonwebtoken::{encode, decode, Header, EncodingKey, DecodingKey, Validation, errors::Error as JwtError};
use std::env;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct User {
    pub id: Option<String>,
    pub username: String,
    pub email: String,
    pub password: String,
    pub role: Role,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Role {
    Admin,
    User,
    Guest,
}

#[derive(Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub role: Role,
    pub exp: i64,
}

impl Claims {
    pub fn new(username: String, role: Role) -> Self {
        Self {
            sub: username,
            role,
            exp: (Utc::now() + Duration::hours(24)).timestamp(),
        }
    }

    pub fn encode(&self) -> Result<String, JwtError> {
        let secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set");
        encode(&Header::default(), self, &EncodingKey::from_secret(secret.as_ref()))
    }

    pub fn decode(token: &str) -> Result<Self, JwtError> {
        let secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set");
        decode::<Claims>(token, &DecodingKey::from_secret(secret.as_ref()), &Validation::default()).map(|data| data.claims)
    }
}