use actix_web::web;

use serde::{Deserialize, Serialize};
use sqlx::{FromRow, PgPool, Postgres, QueryBuilder};
use tracing::instrument;

// use crate::routes::yak;
#[derive(Deserialize, Serialize, FromRow)]
pub struct Yak {
    id: i32,
    name: String,
    age: f32,
    age_last_shaved: f32,
}

#[instrument]
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

#[instrument]
pub async fn pgsql_fetch_yak(pgsql: web::Data<PgPool>, id: i32) -> Result<Yak, sqlx::Error> {
    let query: QueryBuilder<Postgres> =
        QueryBuilder::new("SELECT id,name,age from yak WHERE id = $1");
    match sqlx::query_as::<Postgres, Yak>(query.sql())
        .bind(id)
        .fetch_one(pgsql.as_ref())
        .await
    {
        Ok(yak) => Ok(yak),
        Err(err) => {
            tracing::error!("Error: {}", err);
            Err(err)
        }
    }
}

#[instrument]
pub async fn pgsql_create_yak(
    pgsql: web::Data<PgPool>,
    name: String,
    age: f32,
) -> Result<(), sqlx::Error> {
    let query: QueryBuilder<Postgres> =
        QueryBuilder::new("INSERT INTO yak (name, age) VALUES ($1, $2)");
    match sqlx::query(&query.sql())
        .bind(name)
        .bind(age)
        .execute(pgsql.as_ref())
        .await
    {
        Ok(_) => Ok(()),
        Err(err) => {
            tracing::error!("Error: {}", err);
            Err(err)
        }
    }
}

#[instrument]
pub async fn pgsql_delete_yak(pgsql: web::Data<PgPool>, id: i32) -> Result<(), sqlx::Error> {
    let query: QueryBuilder<Postgres> = QueryBuilder::new("DELETE FROM yak WHERE id = $1");
    match sqlx::query(query.sql())
        .bind(id)
        .execute(pgsql.as_ref())
        .await
    {
        Ok(_) => Ok(()),
        Err(err) => {
            tracing::error!("Error: {}", err);
            Err(err)
        }
    }
}

#[instrument]
pub async fn pgsql_update_yak(
    pgsql: web::Data<PgPool>,
    id: i32,
    name: String,
    age: f32,
) -> Result<(), sqlx::Error> {
    let query: QueryBuilder<Postgres> =
        QueryBuilder::new("UPDATE yak SET age = $1, name = $2 WHERE id = $3");
    match sqlx::query(query.sql())
        .bind(age)
        .bind(name)
        .bind(id)
        .execute(pgsql.as_ref())
        .await
    {
        Ok(_) => Ok(()),
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
