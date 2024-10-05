use crate::db::{conn, DbPool}; // Import your connection pool

use actix_web::{web, App, HttpServer};

mod db; // Declare the db module
mod models; // Declare the models module
mod signup; // Declare the signup module

use dotenvy::dotenv;
use signup::signup;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok(); // Load environment variables from .env file

    let pool = conn(); // Create a database connection pool

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone())) // Share the database connection pool
            .service(signup) // Register the signup route
    })
    .bind("127.0.0.1:8080")? // Bind to localhost on port 8080
    .run()
    .await
}
