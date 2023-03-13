use crate::db::model::user;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct UserResponse {
    username: String,
    registered_at: NaiveDateTime,
}

impl UserResponse {
    pub fn from_db_model(db_user: &user::User) -> Self {
        Self {
            username: db_user.username.clone(),
            registered_at: db_user.created_at,
        }
    }
}

#[derive(Deserialize)]
pub struct CreateUserPayload {
    pub username: String,
    pub password: String,
}

#[derive(Deserialize)]
pub struct UpdateUserPayload {
    pub id: i32,
    pub username: String,
}

#[derive(Deserialize)]
pub struct DeleteUserPayload {
    pub id: i32,
}
