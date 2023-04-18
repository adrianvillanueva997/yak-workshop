use actix_web::HttpResponse;

/// Health check endpoint.
pub async fn health() -> HttpResponse {
    HttpResponse::Ok().finish()
}
