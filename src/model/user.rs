use std::time::SystemTime;
use serde::{Deserialize, Serialize};
use diesel::prelude::*;
use crate::db::schema::users;
use diesel::Queryable;
use crate::helper::serializer::system_time_serializer::{serialize_system_time,deserialize_system_time};

#[derive(Debug, Queryable, Identifiable,Serialize,Deserialize,Insertable)]
#[diesel(table_name = users)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub password: String,
    #[serde(serialize_with = "serialize_system_time")]
    pub created_at: SystemTime
}

#[derive(Insertable,AsChangeset,Deserialize,Serialize)]
#[table_name = "users"]
pub struct NewUser {
    pub name: String,
    pub email: String,
    pub password: String,
    #[serde(deserialize_with = "deserialize_system_time")]
    pub created_at: SystemTime,
}

