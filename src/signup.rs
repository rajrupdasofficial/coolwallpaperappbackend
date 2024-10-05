use crate::db::{conn, DbPool}; // Import your connection pool
use crate::models::NewUser;
use actix_web::{post, web, HttpResponse, Responder};
use argon2::{self, Config};
use diesel::prelude::*;
use rand::Rng;
use serde::Deserialize; // Import your models

#[derive(Deserialize)]
pub struct SignupRequest {
    email: String,
    password: String,
}

#[post("/signup")]
pub async fn signup(user: web::Json<SignupRequest>, pool: web::Data<DbPool>) -> impl Responder {
    // Generate a random salt
    let salt: [u8; 32] = rand::thread_rng().gen();

    // Configure Argon2 parameters
    let config = Config::default();

    // Hash the password
    let password_hash = argon2::hash_encoded(user.password.as_bytes(), &salt, &config)
        .map_err(|_| HttpResponse::InternalServerError().body("Failed to hash password"))?;

    // Get a connection from the pool
    let conn = pool
        .get()
        .expect("Failed to get a connection from the pool");

    // Create a new user instance
    let new_user = NewUser {
        email: &user.email,
        password_hash: &password_hash,
    };

    // Insert the new user into the database
    diesel::insert_into(users::table) // Assuming you have imported users from schema.rs
        .values(&new_user)
        .execute(&*conn)
        .map_err(|_| HttpResponse::InternalServerError().body("Failed to insert user"))?;

    HttpResponse::Created().json({
        serde_json::json!({
            "email": user.email,
            "message": "User created successfully",
        })
    })
}
