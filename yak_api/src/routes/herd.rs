use actix_web::{
    web::{self, Path},
    HttpResponse,
};
use redis::Client;
use sqlx::PgPool;

use crate::dal;

/// Returns the current herd after N days.
///
/// # Panics
///
/// Panics if the database connection pool cannot be retrieved.
pub async fn herd(
    days: Path<i32>,
    pgsql: web::Data<PgPool>,
    redis: web::Data<Client>,
) -> HttpResponse {
    let mut yaks = dal::yak::redis_fetch_all_yaks(redis.clone()).await.unwrap();
    if yaks.len() == 0 {
        yaks = dal::yak::pgsql_fetch_all_yaks(pgsql).await.unwrap();
    }
    if yaks.is_empty() {
        return HttpResponse::NotFound().body("No yaks found");
    }
    let filtered_yaks = yaks
        .into_iter()
        .filter(|yak| yak.days_to_years(*days as f32) <= 10.0)
        .collect::<Vec<_>>();
    HttpResponse::Ok().json(filtered_yaks)
}
