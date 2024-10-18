use std::time::SystemTime;

use serde::{Deserialize, Serialize};
use diesel::prelude::*;
use crate::db::schema::users;

use diesel::Queryable;


#[derive(Debug, Queryable, Identifiable,Serialize,Deserialize,Insertable)]
#[diesel(table_name = users)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub created_at: SystemTime
}

#[derive(Insertable)]
#[table_name = "users"]
pub struct NewUser<'a> {
    pub name: &'a str,
    pub email: &'a str,
    pub created_at: SystemTime,
}

