use std::env;

pub fn get_server_port() -> String {
    env::var("SERVER_PORT").unwrap_or_else(|_| "3000".to_string())
}

pub fn get_database_url() -> String {
    env::var("DATABASE_URL").expect("DATABASE_URL must be set")
}