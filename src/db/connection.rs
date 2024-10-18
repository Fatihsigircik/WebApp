use std::env;
use diesel::RunQueryDsl;
use diesel::{Connection, PgConnection, QueryResult};
use dotenvy::dotenv;

use crate::model::user::{NewUser, User};

use super::schema::users;

fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

pub fn add_user(user:NewUser) -> QueryResult<usize>
{
    let mut conn = establish_connection();
    diesel::insert_into(users::table)
        .values(user)
        .execute(&mut conn)
}

pub fn get_users() -> QueryResult<Vec<User>>
{
    let mut conn = establish_connection();
    let x = users::table.load::<User>(&mut conn);
    return x;
}