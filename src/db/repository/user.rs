use crate::db::{
    model::user::{NewUser, User},
    PgPooledConn,
};
use anyhow::Result;
use diesel::{delete, insert_into, prelude::*, update};
pub trait UserRepository {
    fn create_user(&mut self, user: &NewUser) -> Result<User>;
    fn read_users(&mut self) -> Result<Vec<User>>;
    fn read_user_by_credentials(
        &mut self,
        username: &str,
        encrypted_password: &str,
    ) -> Result<Option<User>>;
    fn update_user(&mut self, user_id: i32, user: &User) -> Result<User>;
    fn remove_user(&mut self, user_id: i32) -> Result<()>;
}

impl UserRepository for PgPooledConn {
    fn create_user(&mut self, new_user: &NewUser) -> Result<User> {
        use crate::schema::users;
        let res = insert_into(users::table)
            .values(new_user)
            .get_result(self)?;
        Ok(res)
    }

    fn read_users(&mut self) -> Result<Vec<User>> {
        use crate::schema::users::dsl::*;
        let res = users.filter(deleted_at.is_null()).load(self)?;
        Ok(res)
    }

    fn read_user_by_credentials(
        &mut self,
        d_username: &str,
        d_encrypted_password: &str,
    ) -> Result<Option<User>> {
        use crate::schema::users::dsl::*;
        let res = users
            .filter(deleted_at.is_null())
            .filter(username.eq(d_username))
            .filter(encrypted_password.eq(d_encrypted_password))
            .first(self)
            .optional()?;
        Ok(res)
    }

    fn update_user(&mut self, user_id: i32, user: &User) -> Result<User> {
        use crate::schema::users::dsl::*;
        let res = update(users.filter(id.eq(user_id)))
            .set(username.eq(&user.username))
            .get_result(self)?;
        Ok(res)
    }

    fn remove_user(&mut self, user_id: i32) -> Result<()> {
        use crate::schema::users::dsl::*;
        delete(users.filter(id.eq(user_id))).execute(self)?;
        Ok(())
    }
}
