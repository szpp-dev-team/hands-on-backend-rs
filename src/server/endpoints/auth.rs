use std::ops::Add;

use crate::{
    db::{repository::user::UserRepository, PgPool},
    server::{
        middleware::auth::{Claims, SECRET},
        model::auth::{SigninPayload, SigninResponse},
    },
    util::password::hash_password,
};
use actix_web::{error::ErrorInternalServerError, post, web, HttpResponse};
use chrono::Local;
use jsonwebtoken::{encode, EncodingKey, Header};

#[post("/auth/signin")]
pub async fn handle_signin(
    pool: web::Data<PgPool>,
    payload: web::Json<SigninPayload>,
) -> Result<HttpResponse, actix_web::Error> {
    let mut conn = pool.get().unwrap();
    let encrypted_password = hash_password(&payload.password);
    let user = conn
        .read_user_by_credentials(&payload.username, &encrypted_password)
        .map_err(ErrorInternalServerError)?;
    let user = match user {
        Some(user) => user,
        None => return Ok(HttpResponse::Unauthorized().finish()),
    };
    let claims = Claims {
        exp: Local::now().add(chrono::Duration::days(7)).timestamp() as usize,
        id: user.id,
    };
    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(SECRET.as_bytes()),
    )
    .map_err(ErrorInternalServerError)?;
    let resp = SigninResponse::new(token);
    Ok(HttpResponse::Ok().json(&resp))
}
