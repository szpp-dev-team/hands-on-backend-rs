use crate::schema::users;
use chrono::NaiveDateTime;
use diesel::prelude::*;

#[derive(Queryable)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub encrypted_password: String,
    pub created_at: NaiveDateTime,
    pub updated_at: Option<NaiveDateTime>,
    pub deleted_at: Option<NaiveDateTime>,
}

#[derive(Insertable)]
#[diesel(table_name = users)]
pub struct NewUser {
    pub username: String,
    pub encrypted_password: String,
    pub created_at: NaiveDateTime,
}
