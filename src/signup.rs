use crate::db::DbPool; // Import your connection pool
use crate::models::NewUser; // Ensure you have NewUser defined
use crate::schema::users; // Import your users schema
use actix_web::{post, web, HttpResponse, Responder};
use bcrypt::{hash, DEFAULT_COST}; // Import bcrypt functions
use diesel::prelude::*;
use serde::Deserialize; // Import your models

#[derive(Deserialize)]
pub struct SignupRequest {
    email: String,
    password: String,
}

// Assuming NewUser is defined like this:
#[derive(Insertable)] // Make sure to derive Insertable for Diesel
#[table_name = "users"] // Name of the table in your database
pub struct NewUser {
    pub email: String,
    pub password_hash: String,
}

#[post("/signup")]
pub async fn signup(
    user: web::Json<SignupRequest>,
    pool: web::Data<DbPool>,
) -> Result<HttpResponse, actix_web::Error> {
    // Hash the password using bcrypt
    let password_hash = hash(&user.password, DEFAULT_COST)
        .map_err(|_| HttpResponse::InternalServerError().body("Failed to hash password"))?;

    // Get a connection from the pool
    let conn = pool.get().map_err(|_| {
        HttpResponse::InternalServerError().body("Failed to get a connection from the pool")
    })?;

    // Create a new user instance
    let new_user = NewUser {
        email: user.email.clone(), // Use clone to get ownership
        password_hash,             // Use directly without cloning
    };

    // Insert the new user into the database
    diesel::insert_into(users::table) // Assuming you have imported users from schema.rs
        .values(&new_user)
        .execute(&*conn)
        .map_err(|_| HttpResponse::InternalServerError().body("Failed to insert user"))?;

    Ok(HttpResponse::Created().json({
        serde_json::json!({
            "email": user.email,
            "message": "User created successfully",
        })
    }))
}
