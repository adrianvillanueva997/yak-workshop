//Process an order and returns the allowed stock given the wool and milk that was requested.

use actix_web::HttpResponse;

pub async fn order() -> HttpResponse {
    HttpResponse::Ok().body("OK")
}
