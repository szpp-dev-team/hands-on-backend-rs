use crate::db::model::user;
use chrono::NaiveDateTime;
use serde::Serialize;

#[derive(Serialize)]
pub struct User {
    username: String,
    registered_at: NaiveDateTime,
}

impl User {
    pub fn from_db_model(db_user: &user::User) -> Self {
        Self {
            username: db_user.username.clone(),
            registered_at: db_user.created_at,
        }
    }
}
