use actix_web::{http::Error, web, HttpResponse};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct YakCreate {
    name: String,
    age: u32,
}

#[derive(Deserialize)]
pub struct Info {
    username: String,
}
pub async fn yak(yak: web::Json<YakCreate>) -> HttpResponse {
    HttpResponse::Ok().body("Hello world!")
}

pub async fn index(info: web::Json<Info>) -> Result<String, Error> {
    Ok(format!("Welcome {}!", info.username))
}
