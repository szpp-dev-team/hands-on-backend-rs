use crate::db::{model::user::User, PgPooledConn};
use anyhow::Result;
use diesel::prelude::*;
pub trait UserRepository {
    fn fetch_users(&mut self) -> Result<Vec<User>>;
}

impl UserRepository for PgPooledConn {
    fn fetch_users(&mut self) -> Result<Vec<User>> {
        use crate::schema::users::dsl::*;
        let res = users.filter(id.is_null()).load(self)?;
        Ok(res)
    }
}
