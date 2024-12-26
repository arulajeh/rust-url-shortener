use actix_web::{App, HttpServer};
use dotenvy::dotenv;

mod config;
mod handlers;
mod models;
mod routes;
mod utils;
mod repository;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Load environment variables from .env file
    dotenv().ok();

    // Get server port from .env or default to 3000
    let server_port = config::get_server_port();

    let database_url = config::get_database_url();
    // Initialize shared state with database connection pool
    let db_pool = sqlx::postgres::PgPoolOptions::new()
        .min_connections(1)
        .max_connections(100)
        .connect(&database_url)
        .await
        .expect("Failed to connect to the database.");

    println!("Connected to the database.");

    // Start Actix Web server
    println!("Server running on http://0.0.0.0:{}", server_port);
    HttpServer::new(move || {
        App::new()
            .app_data(actix_web::web::Data::new(db_pool.clone()))
            .configure(routes::short_routes) // Configure routes
    })
        .bind(format!("0.0.0.0:{}", server_port))?
        .run()
        .await
}