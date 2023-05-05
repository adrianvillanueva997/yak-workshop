//Process an order and returns the allowed stock given the wool and milk that was requested.

use actix_web::{web, HttpResponse};
use redis::Client;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use utoipa::ToSchema;

use crate::dal;
use tracing::instrument;
#[derive(Debug, ToSchema, Serialize, Deserialize)]
struct Response {
    milk: f32,
    wool: f32,
}

#[instrument]
pub async fn order(
    days: web::Path<f32>,
    pgsql: web::Data<PgPool>,
    redis: web::Data<Client>,
) -> HttpResponse {
    let days_val = *days;
    if days_val < 0.0 {
        tracing::error!("Days must be positive");
        return HttpResponse::BadRequest().body("Days must be positive");
    }
    let mut yaks = dal::yak::fetch_yaks(&redis, &pgsql).await.unwrap();
    let (total_milk, total_wool) = yaks
        .iter_mut()
        .map(|yak| yak.production(days_val))
        .fold((0.0, 0.0), |(total_milk, total_wool), (milk, wool)| {
            (total_milk + milk, total_wool + wool)
        });

    let response = Response {
        milk: total_milk,
        wool: total_wool,
    };

    HttpResponse::Ok().json(response)
}
