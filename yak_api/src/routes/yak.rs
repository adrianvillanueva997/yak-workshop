use actix_web::{web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, PgPool};

#[derive(Deserialize, Serialize, FromRow)]
pub struct YakCreate {
    id: i32,
    name: String,
    age: f32,
}

pub async fn create_yak(yak: web::Json<YakCreate>) -> HttpResponse {
    HttpResponse::Ok().body("Hello world!")
}

pub async fn get_yaks(pgsql: web::Data<PgPool>) -> impl Responder {
    match sqlx::query_as::<_, YakCreate>("SELECT id, name, age FROM yak")
        .fetch_all(pgsql.get_ref())
        .await
    {
        Ok(users) => HttpResponse::Ok().json(users),
        Err(_) => HttpResponse::NotFound().json("No users found"),
    }
}
