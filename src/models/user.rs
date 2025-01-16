use chrono::NaiveDateTime;
use diesel::prelude::*;

#[derive(Queryable)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub last_name: String,
    pub email: String,
    pub password: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Insertable)]
#[diesel(table_name=users)]