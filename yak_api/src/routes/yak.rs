use actix_web::{web, HttpResponse, Responder};

use log::error;
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, PgPool, Postgres, QueryBuilder};

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
#[derive(Debug, Deserialize, Serialize)]
pub struct YakDelete {
    id: i32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct YakUpdate {
    id: i32,
    name: String,
    age: f32,
}

pub async fn create_yak(yak: web::Json<YakCreate>, pgsql: web::Data<PgPool>) -> HttpResponse {
    let query: QueryBuilder<Postgres> =
        QueryBuilder::new("INSERT INTO yak (name, age) VALUES ($1, $2)");
    match sqlx::query(&query.sql())
        .bind(&yak.name)
        .bind(yak.age)
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

pub async fn delete_yak(yak: web::Json<YakDelete>, pgsql: web::Data<PgPool>) -> HttpResponse {
    let query: QueryBuilder<Postgres> = QueryBuilder::new("DELETE FROM yak WHERE id = $1");
    match sqlx::query(query.sql())
        .bind(yak.id)
        .execute(pgsql.as_ref())
        .await
    {
        Ok(_) => HttpResponse::Ok().body("OK"),
        Err(err) => {
            error!("Error: {}", err);
            HttpResponse::InternalServerError().body("Error deleting yak")
        }
    }
}

pub async fn update_yak(yak: web::Json<YakUpdate>, pgsql: web::Data<PgPool>) -> HttpResponse {
    let query: QueryBuilder<Postgres> =
        QueryBuilder::new("UPDATE yak SET age = $1, name = $2 WHERE id = $3");
    match sqlx::query(query.sql())
        .bind(yak.age)
        .bind(&yak.name)
        .bind(yak.id)
        .execute(pgsql.as_ref())
        .await
    {
        Ok(_) => HttpResponse::Ok().body("OK"),
        Err(err) => {
            error!("Error: {}", err);
            HttpResponse::InternalServerError().body("Error updating yak")
        }
    }
}
