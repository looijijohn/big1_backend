use actix_web::{web, HttpResponse, Error, ResponseError};
use mongodb::{Client, Collection};
use mongodb::bson::doc;
use bcrypt::{hash, verify, DEFAULT_COST};
use chrono::Utc;
use serde::{Deserialize, Serialize};
use crate::models::{User, NewUser};
use crate::services::find_paginated;
use std::fmt;

#[derive(Debug)]
pub struct ServiceError {
    pub message: String,
}

impl fmt::Display for ServiceError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl ResponseError for ServiceError {}

pub async fn register(client: web::Data<Client>, form: web::Json<NewUser>) -> Result<HttpResponse, Error> {
    let collection: Collection<User> = client.database("user_service").collection("users");
    let password_hash = hash(&form.password_hash, DEFAULT_COST).expect("Failed to hash password");
    let new_user = User {
        id: None,
        username: form.username.clone(),
        email: form.email.clone(),
        password_hash: password_hash,
        created_at: Utc::now(),
    };

    collection.insert_one(new_user, None).await.expect("Error saving new user");

    Ok(HttpResponse::Ok().json("User registered successfully"))
}

pub async fn login(client: web::Data<Client>, form: web::Json<NewUser>) -> Result<HttpResponse, Error> {
    let collection: Collection<User> = client.database("user_service").collection("users");
    let filter = doc! { "email": &form.email };
    let user = collection.find_one(filter, None).await.expect("Error finding user");

    match user {
        Some(user) => {
            if verify(&form.password_hash, &user.password_hash).unwrap() {
                Ok(HttpResponse::Ok().json("Login successful"))
            } else {
                Ok(HttpResponse::Unauthorized().json("Invalid credentials"))
            }
        }
        None => Ok(HttpResponse::Unauthorized().json("Invalid credentials")),
    }
}

pub async fn get_users(
    client: web::Data<Client>,
    query: web::Query<PaginationQuery>,
) -> Result<HttpResponse, Error> {
    let users = find_paginated::<User>(
        &client,
        "user_service",
        "users",
        query.page as u64, // Convert i64 to u64
        query.limit as u64, // Convert i64 to u64
    )
    .await
    .map_err(|e| {
        eprintln!("Error fetching users: {}", e);
        Error::from(ServiceError {
            message: format!("Error fetching users: {}", e),
        })
    })?;

    Ok(HttpResponse::Ok().json(users))
}

#[derive(Deserialize)]
pub struct PaginationQuery {
    page: i64,
    limit: i64,
}