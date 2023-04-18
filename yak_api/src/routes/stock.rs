//Returns the current stock status after N days.

use actix_web::{web, HttpResponse};
use redis::Client;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use tracing::instrument;
use utoipa::ToSchema;

use crate::dal::yak::pgsql_fetch_all_yaks;

#[derive(Debug, ToSchema, Serialize, Deserialize)]
pub struct Stock {
    milk: f32,
    wool: f32,
}

/// Get the current stock status after N days.
#[utoipa::path(
    get,
    path = "/stock/{days}",
    responses(
        (status = 200, description = "Stock status", body = Stock),
        (status = 501, description = "Error getting stock status", body=Err)
    ),
    responses(
        (status = 200, description = "Stock status", body = Stock),
        (status = 501, description = "Error getting stock status", body=Err)
    )
)]
#[instrument]
pub async fn stock(
    days: web::Path<f32>,
    pgsql: web::Data<PgPool>,
    redis: web::Data<Client>,
) -> HttpResponse {
    tracing::info!("Getting stock status after {} days", days);
    if days.clone() < 0.0 {
        tracing::error!("Days must be greater than 0");
        return HttpResponse::BadRequest().body("Days must be greater than 0");
    }
    let yaks = match pgsql_fetch_all_yaks(pgsql).await {
        Ok(yaks) => *yaks,
        Err(err) => {
            tracing::error!("Error: {}", err);
            return HttpResponse::InternalServerError().body("Error getting stock status");
        }
    };
    let mut stock = Stock {
        milk: 0.0,
        wool: 0.0,
    };
    for mut yak in yaks {
        let (milk, wool) = yak.production(*days);
        stock.milk += milk;
        stock.wool += wool;
    }
    HttpResponse::Ok().json(stock)
}
