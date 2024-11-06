use std::time::SystemTime;

use serde::{Deserialize, Serialize};
use diesel::prelude::*;
use crate::db::schema::menus;

use diesel::Queryable;


#[derive(Debug, Queryable, Identifiable,Serialize,Deserialize)]
#[diesel(table_name = menus)]
pub struct Menu {
    pub id: i32,
    pub parent_menu_id: i32,
    pub title: String,
    pub link_type: i8,
    pub link: String,
    pub created_at: SystemTime,
    pub updated_at: SystemTime,
}

#[derive(Insertable)]
#[table_name = "menus"]
pub struct NewMenu<'a> {
    pub parent_menu_id: i32,
    pub title: &'a str,
    pub link_type: i16,
    pub link: &'a str,
    pub created_at: SystemTime,
    pub updated_at: SystemTime,
}

