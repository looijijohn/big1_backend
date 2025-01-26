use actix_web::{web, HttpResponse, Error};
use serde::Deserialize;
use crate::utils::create_jwt;
use crate::errors::AppError;

#[derive(Deserialize)]
pub struct JwtRequest {
    pub user_id: String,
}

pub async fn create_jwt_route(
    jwt_request: web::Json<JwtRequest>,
    jwt_secret: web::Data<String>,
) -> Result<HttpResponse, AppError> {
    let token = create_jwt(&jwt_request.user_id, &jwt_secret)?;
    Ok(HttpResponse::Ok().json(token))
}