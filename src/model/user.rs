use serde::{Deserialize, Serialize};
use diesel::prelude::*;
use crate::db::schema::users;

#[derive(Serialize,Deserialize)]
#[derive(Queryable)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub password: String,
}

#[derive(Insertable)]
#[diesel(table_name = users)]
pub struct NewUser<'a> {
    pub name: &'a str,
    pub email: &'a str,
}
