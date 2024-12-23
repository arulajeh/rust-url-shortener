use actix_web::{
    middleware::Logger, web, App, HttpResponse, HttpServer, Responder,
};
use rand::random;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

type ShortenedUrls = Arc<Mutex<HashMap<String, String>>>;

#[derive(Deserialize)]
struct ShortenRequest {
    url: String,
}

#[derive(Serialize)]
struct ShortenResponse {
    short: String,
}

#[derive(Serialize)]
struct RedirectResponse {
    url: String,
}

#[derive(Serialize)]
struct ErrorResponse {
    error: String,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Initialize shared state
    let urls: ShortenedUrls = Arc::new(Mutex::new(HashMap::new()));

    // Start Actix Web server
    println!("Starting server on port 3000");
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(urls.clone()))
            .wrap(Logger::default())
            .route("/shorten", web::post().to(shorten_url))
            .route("/{short}", web::get().to(redirect_url))
    })
        .bind("0.0.0.0:3000")?
        .run()
        .await
}

async fn shorten_url(
    data: web::Data<ShortenedUrls>,
    req: web::Json<ShortenRequest>,
) -> impl Responder {
    // Generate random short ID
    let short = random::<u64>().to_string();

    // Store original URL and short ID
    let mut urls = data.lock().unwrap();
    urls.insert(short.clone(), req.url.clone());

    // Return the short ID
    HttpResponse::Created().json(ShortenResponse { short })
}

async fn redirect_url(
    data: web::Data<ShortenedUrls>,
    path: web::Path<String>,
) -> impl Responder {
    let urls = data.lock().unwrap();
    if let Some(url) = urls.get(&path.into_inner()) {
        HttpResponse::Ok()
            .json(RedirectResponse { url: url.clone() })
    } else {
        HttpResponse::NotFound().json(ErrorResponse {
            error: "URL not found".to_string(),
        })
    }
}
