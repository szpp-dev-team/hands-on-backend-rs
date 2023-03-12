use crate::{
    db::{repository::user::UserRepository, PgPool},
    server::model::user::User,
};
use actix_web::{error::ErrorInternalServerError, get, web::Data, HttpResponse};

#[get("/users")]
pub async fn handle_get_users(db_pool: Data<PgPool>) -> Result<HttpResponse, actix_web::Error> {
    let mut db_conn = db_pool.get().map_err(ErrorInternalServerError)?;
    let users = db_conn
        .fetch_users()
        .map_err(ErrorInternalServerError)?
        .iter()
        .map(User::from_db_model)
        .collect::<Vec<_>>();
    Ok(HttpResponse::Ok().json(users))
}
