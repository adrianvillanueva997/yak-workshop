use actix_web::{web, HttpResponse, Responder};

use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use tracing::instrument;

use crate::dal::yak::{
    pgsql_create_yak, pgsql_delete_yak, pgsql_fetch_all_yaks, pgsql_fetch_yak,
    redis_fetch_all_yaks, redis_fetch_yak, redis_insert_all_yaks, redis_insert_yak,
};

#[derive(Debug, Deserialize, Serialize)]
pub struct YakCreate {
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

/// .
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
/// .
///
/// # Panics
///
/// Panics if .
#[instrument]
pub async fn get_yaks(pgsql: web::Data<PgPool>, redis: web::Data<redis::Client>) -> impl Responder {
    tracing::info!("Getting yaks");
    let yaks = redis_fetch_all_yaks(redis.clone()).await.unwrap();
    if yaks.len() == 0 {
        match pgsql_fetch_all_yaks(pgsql).await {
            Ok(yaks) => {
                match redis_insert_all_yaks(redis, yaks.clone()).await {
                    Ok(_) => (),
                    Err(err) => {
                        tracing::error!("Error: {}", err);
                    }
                }
                HttpResponse::Ok().json(yaks)
            }
            Err(err) => {
                tracing::error!("Error: {}", err);
                HttpResponse::NotFound().body("No yaks found")
            }
        }
    } else {
        HttpResponse::Ok().json(yaks)
    }
}

/// .
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

/// .
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

/// .
///
/// # Panics
///
/// Panics if .
#[instrument]
pub async fn get_yak(
    id: web::Path<i32>,
    pgsql: web::Data<PgPool>,
    redis: web::Data<redis::Client>,
) -> HttpResponse {
    tracing::info!("Searching yak: {:?}", id);
    let id: i32 = id.into_inner();
    match redis_fetch_yak(redis.clone(), id).await {
        Ok(yak) => HttpResponse::Ok().json(yak),
        Err(err) => {
            tracing::error!("Error: {}", err);
            match pgsql_fetch_yak(pgsql, id).await {
                Ok(yak) => {
                    redis_insert_yak(redis, yak.clone()).await.unwrap();
                    HttpResponse::Ok().json(yak);
                }
                Err(err) => {
                    tracing::error!("Error: {}", err);
                    HttpResponse::NotFound().body("No yak found");
                }
            }
            HttpResponse::NotFound().body("No yak found")
        }
    }
}
