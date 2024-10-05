use crate::schema::users;
use diesel::prelude::*; // Import the users schema

#[derive(Queryable)]
pub struct User {
    pub id: i32,
    pub email: String,
    pub password_hash: String,
}

#[derive(Insertable)]
#[table_name = "users"]
pub struct NewUser<'a> {
    pub email: &'a str,
    pub password_hash: &'a str,
}
