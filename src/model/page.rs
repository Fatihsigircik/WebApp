use std::time::SystemTime;

use serde::{Deserialize, Serialize};
use diesel::prelude::*;
use crate::db::schema::pages;

use diesel::Queryable;


#[derive(Debug, Queryable, Identifiable,Serialize,Deserialize)]
#[diesel(table_name = pages)]
pub struct Page {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub created_at: SystemTime,
    pub updated_at: SystemTime
}

#[derive(Insertable)]
#[table_name = "pages"]
pub struct NewPage<'a> {
    pub title: &'a str,
    pub body: &'a str,
    pub created_at: SystemTime,
    pub updated_at: SystemTime,
}

