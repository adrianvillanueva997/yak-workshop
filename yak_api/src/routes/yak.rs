use actix_web::HttpResponse;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
struct YakRequest {
    name: String,
    age: u32,
}

pub async fn yak() -> HttpResponse {
    HttpResponse::Ok().body("Hello world!")
}
