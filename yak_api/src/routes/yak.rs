use actix_web::{web, HttpResponse, Responder};

use serde::{Deserialize, Serialize};
use sqlx::{FromRow, PgPool};
use tracing::instrument;

use crate::dal::yak::{pgsql_create_yak, pgsql_delete_yak, pgsql_fetch_all_yaks, pgsql_fetch_yak};

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
    match pgsql_create_yak(pgsql, yak.name.to_string(), yak.age).await {
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
    match pgsql_fetch_all_yaks(pgsql).await {
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
    match pgsql_delete_yak(pgsql, yak.id).await {
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
    match pgsql_create_yak(pgsql, yak.name.to_string(), yak.age).await {
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
    match pgsql_fetch_yak(pgsql, id.into_inner()).await {
        Ok(yak) => HttpResponse::Ok().json(yak),
        Err(err) => {
            tracing::error!("Error: {}", err);
            HttpResponse::NotFound().body("No yak found")
        }
    }
}
