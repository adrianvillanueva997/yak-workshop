use actix_web::{web, HttpResponse, Responder};

use log::error;
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, PgPool, Postgres};
#[derive(Debug, Deserialize, Serialize)]
pub struct YakCreate {
    name: String,
    age: f32,
}
#[derive(Deserialize, Serialize, FromRow)]
pub struct Yak {
    id: i32,
    name: String,
    age: f32,
}

pub async fn create_yak(yak: web::Json<YakCreate>, pgsql: web::Data<PgPool>) -> HttpResponse {
    let query_builder: sqlx::QueryBuilder<Postgres> =
        sqlx::QueryBuilder::new("INSERT INTO yak (name, age) VALUES ($1, $2)");
    let sql = query_builder.sql();
    println!("{sql}");
    match sqlx::query(&sql)
        .bind(&yak.name)
        .bind(&yak.age)
        .execute(pgsql.as_ref())
        .await
    {
        Ok(_) => HttpResponse::Ok().body("OK"),
        Err(err) => {
            error!("Error: {}", err);
            HttpResponse::InternalServerError().body("Error creating yak")
        }
    }
}

pub async fn get_yaks(pgsql: web::Data<PgPool>) -> impl Responder {
    match sqlx::query_as::<_, Yak>("SELECT id,name,age from yak")
        .fetch_all(pgsql.get_ref())
        .await
    {
        Ok(users) => HttpResponse::Ok().json(users),
        Err(err) => {
            error!("Error: {}", err);
            HttpResponse::NotFound().body("No yaks found")
        }
    }
}

// pub async fn delete_yak() {
//     unimplemented!()
// }
