use actix_web::web;
use crate::handlers::short::{generate_short_url, redirect_to_original};

/// Konfigurasi routing untuk endpoint short URL
pub fn short_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/short/generate")
            .route(web::post().to(generate_short_url)), // POST untuk membuat URL pendek
    )
        .service(
            web::resource("/short/redirect")
                .route(web::post().to(redirect_to_original)), // POST untuk mendapatkan URL asli
        );
}
