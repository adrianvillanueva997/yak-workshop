use actix_web::{web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use sqlx::{PgPool, Row};

#[derive(Deserialize, Serialize)]
pub struct YakCreate {
    name: String,
    age: u32,
}

pub async fn create_yak(yak: web::Json<YakCreate>) -> HttpResponse {
    HttpResponse::Ok().body("Hello world!")
}

pub async fn get_yaks(pgsql: web::Data<PgPool>) -> impl Responder {
    let row = sqlx::query("select 1 as id")
        .fetch_one(pgsql.get_ref())
        .await
        .unwrap();
    let one1: i32 = row.try_get("id").unwrap();
    format!("{:?}", one1)
}
