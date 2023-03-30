use actix_web::web;
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, PgPool, Postgres, QueryBuilder};

// use crate::routes::yak;
#[derive(Deserialize, Serialize, FromRow)]
pub struct Yak {
    id: i32,
    name: String,
    age: f32,
    age_last_shaved: f32,
}

pub async fn pgsql_fetch_all_yaks(pgsql: web::Data<PgPool>) -> Result<Vec<Yak>, sqlx::Error> {
    match sqlx::query_as::<Postgres, Yak>("SELECT id,name,age, age_last_shaved from yak")
        .fetch_all(pgsql.get_ref())
        .await
    {
        Ok(yaks) => Ok(yaks),
        Err(err) => {
            tracing::error!("Error: {}", err);
            Err(err)
        }
    }
}

pub async fn redis_fetch_all_yaks(
    redis: web::Data<redis::Client>,
) -> Result<Vec<Yak>, redis::RedisError> {
    unimplemented!()
}
