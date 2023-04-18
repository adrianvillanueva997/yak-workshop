use actix_web::{web, HttpResponse};

use redis::Client;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use tracing::instrument;
use utoipa::ToSchema;

use crate::dal::yak::{
    pgsql_create_yak, pgsql_delete_yak, pgsql_fetch_all_yaks, pgsql_fetch_yak,
    redis_fetch_all_yaks, redis_fetch_yak, redis_insert_all_yaks, redis_insert_yak,
};

#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct YakCreate {
    name: String,
    age: f32,
}

#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct YakDelete {
    id: i32,
}

#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct YakUpdate {
    id: i32,
    name: String,
    age: f32,
}

/// Create a new Yak in the database.
#[utoipa::path(
        post,
        path = "/yak",
        responses(
            (status = 200, description = "Yak created succesfully", body = Ok),
            (status = 501, description = "Yak was not created", body=Err)
        ),
        request_body(
            content_type="application/json",
            content=YakCreate,
            description="Request to create a yak",
            examples(
                ("YakCreate"= (description="{\"name\": \"Yak\", \"age\": 1.0}}")),

            )
        ),
    )]
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

/// Gets all yaks from the database.
///
/// # Panics
///
/// Panics if there are no yaks in the database.
#[utoipa::path(
        get,
        path = "/yak",
        responses(
            (status = 200, description = "Yaks found", body = Vec<Yak>),
            (status = 404, description = "No yaks found")
        ),
        tag = "Yak"

    )]
#[instrument]
pub async fn get_yaks(pgsql: web::Data<PgPool>, redis: web::Data<Client>) -> HttpResponse {
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
                HttpResponse::Ok().json(*yaks)
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

/// Deletes a yak from the database.
#[utoipa::path(
    delete,
    path = "/yak",
    responses(
        (status = 200, description = "Yak found", body = Yak),
        (status = 501, description = "No yak found")
    ),
    tag="Yak"
)]
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

/// Updates a yak in the database.
#[utoipa::path(
    put,
    path = "/yak",
    responses(
        (status = 200, description = "Yak found", body = Yak),
        (status = 501, description = "No yak found")
    ),
    request_body(
        content_type="application/json",
        content=YakUpdate,
        description="Request to update a yak",
        examples(
            ("YakUpdate"= (description="{\"id\": 1, \"name\": \"Yak\", \"age\": 1.0}}")),

        )
    ),
    tag="Yak"
)]
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

/// Gets a single yak from the database.
///
/// # Panics
///
/// Panics if there is no yak with the given id.
#[utoipa::path(
    get,
    path = "/yak/{id}",
    responses(
        (status = 200, description = "Yak found", body = Yak),
        (status = 404, description = "No yak found")
    ),
    tag="Yak",
)]
#[instrument]
pub async fn get_yak(
    id: web::Path<i32>,
    pgsql: web::Data<PgPool>,
    redis: web::Data<Client>,
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
