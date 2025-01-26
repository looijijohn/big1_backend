use actix_web::{Error, HttpRequest, HttpResponse, web};
use actix_web_httpauth::extractors::bearer::BearerAuth;
use crate::{errors::AppError, utils::decode_jwt};
 

pub async fn validate_jwt(req: HttpRequest, credentials: BearerAuth) -> Result<HttpResponse, AppError> {
    let jwt_secret = req.app_data::<web::Data<String>>().unwrap();
    decode_jwt(credentials.token(), &jwt_secret)?;
    Ok(HttpResponse::Ok().finish())
}