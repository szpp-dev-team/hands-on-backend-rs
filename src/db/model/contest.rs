use chrono::NaiveDateTime;

pub struct Contest {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub start_at: NaiveDateTime,
    pub end_at: NaiveDateTime,
    pub penalty: i32,
    pub created_at: NaiveDateTime,
    pub updated_at: Option<NaiveDateTime>,
    pub deleted_at: Option<NaiveDateTime>,
}
