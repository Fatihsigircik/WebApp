
use diesel::query_dsl::methods::{FilterDsl, FindDsl};
use diesel::{ExpressionMethods, QueryResult, RunQueryDsl};
use crate::db::connection::establish_connection;
use crate::model::user::{NewUser, User};
use crate::db::schema::users;

pub fn add_user(user:NewUser) -> QueryResult<usize>
{
    let mut conn = establish_connection();
    diesel::insert_into(users::table)
        .values(user)
        .execute(&mut conn)
}

pub async fn get_user_by_id(id: i32) -> QueryResult<User> {
    let mut conn = establish_connection();
    let x = users::table.find(id).get_result::<User>(&mut conn);
    return x;
}

pub async fn get_all_users() -> QueryResult<Vec<User>> {
    let mut conn = establish_connection();
    let x = users::table.load::<User>(&mut conn);
    return x;
}

pub async fn update_user(id: i32, user:NewUser) -> QueryResult<usize> {
    let mut conn = establish_connection();
    diesel::update(users::table)
        .set(user)
        .filter(users::id.eq(id))
        .execute(&mut conn)
}

pub async fn delete_user(id: i32) -> QueryResult<usize> {
    let mut conn = establish_connection();
    diesel::delete(users::table)
        .filter(users::id.eq(id))
        .execute(&mut conn)
}

pub async fn get_user_by_email(email: String) -> QueryResult<User> {
    let mut conn = establish_connection();
    let x = users::table.filter(users::email.eq(email)).first(&mut conn);
    return x;
}

pub async fn get_user_by_email_and_password(email: String,password: String) -> QueryResult<User> {
    let mut conn = establish_connection();
    let x = users::table
    .filter(users::email.eq(email))
    .filter(users::password.eq(password))
    .first(&mut conn);
    return x;
}
