use actix_web::{web, HttpResponse, Responder};

use serde::{Deserialize, Serialize};
use sqlx::{FromRow, PgPool, Postgres, QueryBuilder};
use tracing::instrument;

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
    age_last_shaved: f32,
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

#[instrument]
pub async fn create_yak(yak: web::Json<YakCreate>, pgsql: web::Data<PgPool>) -> HttpResponse {
    tracing::info!("Creating yak: {:?}", yak);
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
            tracing::error!("Error: {}", err);
            HttpResponse::InternalServerError().body("Error creating yak")
        }
    }
}
#[instrument]
pub async fn get_yaks(pgsql: web::Data<PgPool>) -> impl Responder {
    tracing::info!("Getting yaks");
    match sqlx::query_as::<Postgres, Yak>("SELECT id,name,age from yak")
        .fetch_all(pgsql.get_ref())
        .await
    {
        Ok(yaks) => HttpResponse::Ok().json(yaks),
        Err(err) => {
            tracing::error!("Error: {}", err);
            HttpResponse::NotFound().body("No yaks found")
        }
    }
}

#[instrument]
pub async fn delete_yak(yak: web::Json<YakDelete>, pgsql: web::Data<PgPool>) -> HttpResponse {
    tracing::info!("Deleting yak: {:?}", yak);
    let query: QueryBuilder<Postgres> = QueryBuilder::new("DELETE FROM yak WHERE id = $1");
    match sqlx::query(query.sql())
        .bind(yak.id)
        .execute(pgsql.as_ref())
        .await
    {
        Ok(_) => HttpResponse::Ok().body("OK"),
        Err(err) => {
            tracing::error!("Error: {}", err);
            HttpResponse::InternalServerError().body("Error deleting yak")
        }
    }
}

#[instrument]
pub async fn update_yak(yak: web::Json<YakUpdate>, pgsql: web::Data<PgPool>) -> HttpResponse {
    tracing::info!("Updating yak: {:?}", yak);
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
            tracing::error!("Error: {}", err);
            HttpResponse::InternalServerError().body("Error updating yak")
        }
    }
}

#[instrument]
pub async fn get_yak(id: web::Path<i32>, pgsql: web::Data<PgPool>) -> HttpResponse {
    tracing::info!("Searching yak: {:?}", id);
    let query: QueryBuilder<Postgres> =
        QueryBuilder::new("SELECT id,name,age from yak WHERE id = $1");
    match sqlx::query_as::<Postgres, Yak>(query.sql())
        .bind(id.as_ref())
        .fetch_one(pgsql.as_ref())
        .await
    {
        Ok(yak) => HttpResponse::Ok().json(yak),
        Err(err) => {
            tracing::error!("Error: {}", err);
            HttpResponse::InternalServerError().body("Error yak not found")
        }
    }
}
