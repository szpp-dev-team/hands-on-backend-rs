use crate::schema::tasks;
use chrono::NaiveDateTime;
use diesel::prelude::*;

#[derive(Queryable)]
pub struct Task {
    pub id: i32,
    pub author_id: i32,
    pub contest_id: i32,
    pub name: String,
    pub statement: String,
    pub constraints: String,
    pub input: String,
    pub output: String,
    pub score: i32,
    pub time_limit: i32,
    pub memory_limit: i32,
    pub created_at: NaiveDateTime,
    pub updated_at: Option<NaiveDateTime>,
    pub deleted_at: Option<NaiveDateTime>,
}

#[derive(Insertable)]
#[diesel(table_name = tasks)]
pub struct NewTask {
    pub author_id: i32,
    pub contest_id: i32,
    pub name: String,
    pub statement: String,
    pub constraints: String,
    pub input: String,
    pub output: String,
    pub score: i32,
    pub time_limit: i32,
    pub memory_limit: i32,
    pub created_at: NaiveDateTime,
}
