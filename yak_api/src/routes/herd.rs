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
    yaks.retain(|yak| {
        let new_age = yak.days_to_years(*days as f32);
        if new_age <= 10.0 {
            true
        } else {
            false
        }
    });
    HttpResponse::Ok().json(yaks)
}
