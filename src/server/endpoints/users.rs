use crate::{
    db::{repository::user::UserRepository, PgPool},
    server::{
        middleware::auth::Claims,
        model::user::{UpdateUserPayload, UserResponse},
    },
};
use actix_web::{
    error::ErrorInternalServerError,
    get, put,
    web::{self, Data},
    HttpResponse,
};
use diesel::Connection;

#[get("/users")]
pub async fn handle_get_users(db_pool: Data<PgPool>) -> Result<HttpResponse, actix_web::Error> {
    let mut db_conn = db_pool.get().map_err(ErrorInternalServerError)?;
    let users = db_conn
        .read_users()
        .map_err(ErrorInternalServerError)?
        .iter()
        .map(UserResponse::from_db_model)
        .collect::<Vec<_>>();
    Ok(HttpResponse::Ok().json(users))
}

#[put("/users")]
pub async fn handle_update_user(
    db_pool: Data<PgPool>,
    claims: Claims,
    payload: web::Json<UpdateUserPayload>,
) -> Result<HttpResponse, actix_web::Error> {
    if claims.id != payload.id {
        return Ok(HttpResponse::Unauthorized().finish());
    }
    let mut db_conn = db_pool.get().map_err(ErrorInternalServerError)?;
    let user = db_conn
        .transaction(|conn| {
            let mut user = conn.read_user_by_id(payload.id)?;
            user.username = payload.username.clone();
            conn.update_user(payload.id, &user)?;
            Ok::<_, anyhow::Error>(user)
        })
        .map_err(ErrorInternalServerError)?;
    Ok(HttpResponse::Ok().json(UserResponse::from_db_model(&user)))
}
