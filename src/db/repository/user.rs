use crate::db::{
    model::user::{NewUser, User},
    PgPooledConn,
};
use anyhow::Result;
use diesel::{delete, insert_into, prelude::*, update};
pub trait UserRepository {
    fn create_user(&mut self, user: &NewUser) -> Result<User>;
    fn read_users(&mut self) -> Result<Vec<User>>;
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
        let res = users.filter(id.is_null()).load(self)?;
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
