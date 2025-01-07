use crate::models::{
    request::redirect_request::RedirectRequest,
    request::shorten_request::ShortenRequest,
    response::redirect_response::RedirectResponse,
    response::response::ApiResponse,
    response::shorten_response::ShortenResponse,
};
use crate::repository::url_repository;
use crate::utils::generate_random_string;
use actix_web::{web, HttpResponse, Responder};

/// Handler untuk generate URL pendek
pub async fn generate_short_url(
    pool: web::Data<sqlx::PgPool>,
    req: web::Json<ShortenRequest>,
) -> impl Responder {
    let short = generate_random_string();
    if let Err(e) = url_repository::insert_url(pool.get_ref(), &short, &req.url).await {
        return HttpResponse::InternalServerError().json(ApiResponse::<()>::error(
            &format!("Failed to insert URL: {}", e),
            "ERROR500",
        ));
    }
    HttpResponse::Created().json(ApiResponse::success(
        "URL shortened successfully",
        "SUCCESS201",
        Some(ShortenResponse { short }),
    ))
}

/// Handler untuk redirect URL asli
pub async fn redirect_to_original(
    pool: web::Data<sqlx::PgPool>,
    req: web::Json<RedirectRequest>,
) -> impl Responder {
    match url_repository::get_url_by_shorten(pool.get_ref(), &req.short).await {
        // Jika URL ditemukan
        Ok(Some(record)) => {
            // Update counter
            if let Err(e) = url_repository::update_counter(pool.get_ref(), &req.short).await {
                return HttpResponse::InternalServerError().json(ApiResponse::<()>::error(
                    &format!("Failed to update counter: {}", e),
                    "ERROR500",
                ));
            }
            HttpResponse::Ok().json(ApiResponse::success(
                "Redirect URL found",
                "SUCCESS200",
                Some(RedirectResponse {
                    original_url: record.url,
                }),
            ))
        },

        // Jika URL tidak ditemukan
        Ok(None) => HttpResponse::NotFound().json(ApiResponse::<()>::error(
            "Shortened URL not found",
            "ERROR404",
        )),

        // Jika terjadi error saat query
        Err(e) => HttpResponse::InternalServerError().json(ApiResponse::<()>::error(
            &format!("Failed to query URL: {}", e),
            "ERROR500",
        )),
    }
}