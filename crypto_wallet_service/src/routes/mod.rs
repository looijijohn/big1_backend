use actix_web::{web, HttpResponse, Error};
use actix_web_httpauth::extractors::bearer::BearerAuth;
use serde::Deserialize;
use crate::models::Wallet;
use crate::services::{create_wallet, get_wallet, update_balance, delete_wallet};
use crate::utils::decode_jwt;
use crate::errors::AppError;

#[derive(Deserialize)]
pub struct CreateWalletRequest {
    pub currency: String,
}

pub async fn create_wallet_route(
    db: web::Data<mongodb::Database>,
    auth: BearerAuth,
    jwt_secret: web::Data<String>,
    currency: web::Json<CreateWalletRequest>,
) -> Result<HttpResponse, AppError> {
    let claims = decode_jwt(auth.token(), &jwt_secret)?;
    let user_id = claims.sub;

    // Unwrap the Data<Database> to get the underlying Database
    let db = db.get_ref().clone();

    // Create a new wallet
    let wallet = create_wallet(db, user_id, currency.into_inner().currency).await?;
    Ok(HttpResponse::Created().json(wallet))
}

pub async fn get_wallet_route(
    db: web::Data<mongodb::Database>,
    auth: BearerAuth,
    jwt_secret: web::Data<String>,
) -> Result<HttpResponse, AppError> {
    let claims = decode_jwt(auth.token(), &jwt_secret)?;
    let user_id = claims.sub;

    // Unwrap the Data<Database> to get the underlying Database
    let db = db.get_ref().clone();

    // Get the wallet
    let wallet = get_wallet(db, user_id).await?;
    Ok(HttpResponse::Ok().json(wallet))
}

pub async fn update_balance_route(
    db: web::Data<mongodb::Database>,
    auth: BearerAuth,
    jwt_secret: web::Data<String>,
    amount: web::Json<f64>,
) -> Result<HttpResponse, AppError> {
    let claims = decode_jwt(auth.token(), &jwt_secret)?;
    let user_id = claims.sub;

    // Unwrap the Data<Database> to get the underlying Database
    let db = db.get_ref().clone();

    // Update the wallet balance
    update_balance(db, user_id, amount.into_inner()).await?;
    Ok(HttpResponse::Ok().body("Balance updated"))
}

pub async fn delete_wallet_route(
    db: web::Data<mongodb::Database>,
    auth: BearerAuth,
    jwt_secret: web::Data<String>,
) -> Result<HttpResponse, AppError> {
    let claims = decode_jwt(auth.token(), &jwt_secret)?;
    let user_id = claims.sub;

    // Unwrap the Data<Database> to get the underlying Database
    let db = db.get_ref().clone();

    // Delete the wallet
    delete_wallet(db, user_id).await?;
    Ok(HttpResponse::Ok().body("Wallet deleted"))
}