use actix_web::web;

use redis::FromRedisValue;
use sqlx::{PgPool, Postgres, QueryBuilder};
use tracing::instrument;

use crate::models::yak::Yak;

impl FromRedisValue for Yak {
    fn from_redis_value(v: &redis::Value) -> redis::RedisResult<Self> {
        let value: String = FromRedisValue::from_redis_value(v)?;
        let yak: Yak = serde_json::from_str(&value).unwrap();
        Ok(yak)
    }
}

/// Fetches all yaks from Postgres database.
///
/// # Errors
///
/// This function will return an error if the database query fails.
#[instrument]
pub async fn pgsql_fetch_all_yaks(pgsql: web::Data<PgPool>) -> Result<Box<Vec<Yak>>, sqlx::Error> {
    match sqlx::query_as::<Postgres, Yak>("SELECT id,name,age, age_last_shaved from yak")
        .fetch_all(pgsql.get_ref())
        .await
    {
        Ok(yaks) => Ok(Box::new(yaks)),
        Err(err) => {
            tracing::error!("Error: {}", err);
            Err(err)
        }
    }
}

/// Fetches a yak from Postgres database.
///
/// # Errors
///
/// This function will return an error if the database query fails.
#[instrument]
pub async fn pgsql_fetch_yak(pgsql: web::Data<PgPool>, id: i32) -> Result<Box<Yak>, sqlx::Error> {
    let query: QueryBuilder<Postgres> =
        QueryBuilder::new("SELECT id,name,age, age_last_shaved from yak WHERE id = $1");
    match sqlx::query_as::<Postgres, Yak>(query.sql())
        .bind(id)
        .fetch_one(pgsql.as_ref())
        .await
    {
        Ok(yak) => Ok(Box::new(yak)),
        Err(err) => {
            tracing::error!("Error: {}", err);
            Err(err)
        }
    }
}

/// Creates a new yak in the Postgres database.
///
/// # Errors
///
/// This function will return an error if the database query fails.
#[instrument]
pub async fn pgsql_create_yak(
    pgsql: web::Data<PgPool>,
    name: String,
    age: f32,
) -> Result<(), sqlx::Error> {
    static SQL: &str = "INSERT INTO yak (name, age) VALUES ($1, $2)";
    let query: QueryBuilder<Postgres> = QueryBuilder::new(SQL);
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

/// Deletes a yak from the Postgres database.
///
/// # Errors
///
/// This function will return an error if the database query fails.
#[instrument]
pub async fn pgsql_delete_yak(pgsql: web::Data<PgPool>, id: i32) -> Result<(), sqlx::Error> {
    static SQL: &str = "DELETE FROM yak WHERE id = $1";
    let query: QueryBuilder<Postgres> = QueryBuilder::new(SQL);
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

/// Updates a yak in the Postgres database.
///
/// # Errors
///
/// This function will return an error if the database query fails.
#[instrument]
pub async fn pgsql_update_yak(
    pgsql: web::Data<PgPool>,
    id: i32,
    name: String,
    age: f32,
) -> Result<(), sqlx::Error> {
    static SQL: &str = "UPDATE yak SET age = $1, name = $2 WHERE id = $3";
    let query: QueryBuilder<Postgres> = QueryBuilder::new(SQL);
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
/// Fetches all yaks from Redis database.
///
/// # Errors
///
/// This function will return an error if the database query fails.
#[instrument]
pub async fn redis_fetch_all_yaks(
    redis: web::Data<redis::Client>,
) -> Result<Box<Vec<Yak>>, redis::RedisError> {
    let mut connection = redis.get_connection()?;
    let yaks: Vec<Yak> = redis::cmd("LRANGE")
        .arg("yaks")
        .arg(0)
        .arg(-1)
        .query(&mut connection)?;
    Ok(Box::new(yaks))
}
/// Inserts all yaks into Redis database.
///
/// # Panics
///
/// Panics if the Redis connection fails.
///
/// # Errors
///
/// This function will return an error if the database query fails.
#[instrument]
pub async fn redis_insert_all_yaks(
    redis: web::Data<redis::Client>,
    yaks: Box<Vec<Yak>>,
) -> Result<(), redis::RedisError> {
    let redis_connection = redis.get_async_connection().await;
    match redis_connection {
        Ok(mut connection) => {
            for yak in *yaks {
                let yak_json = serde_json::to_string(&yak).unwrap();
                redis::cmd("RPUSH")
                    .arg("yaks")
                    .arg(yak_json)
                    .query_async(&mut connection)
                    .await?;
            }
            redis::cmd("EXPIRE")
                .arg("yaks")
                .arg(60)
                .query_async(&mut connection)
                .await?;
            Ok(())
        }
        Err(err) => {
            tracing::error!("Error: {}", err);
            Err(err)
        }
    }
}
/// Fetches a yak from Redis database.
///
/// # Errors
///
/// This function will return an error if the database query fails.
#[instrument]
pub async fn redis_fetch_yak(
    redis: web::Data<redis::Client>,
    id: i32,
) -> Result<Box<Yak>, redis::RedisError> {
    let redis_connection = redis.get_async_connection().await;
    match redis_connection {
        Ok(mut connection) => {
            let yak_json: Option<String> = redis::cmd("LINDEX")
                .arg("yaks")
                .arg(id)
                .query_async(&mut connection)
                .await?;

            match yak_json {
                Some(json) => {
                    let yak: Yak = serde_json::from_str(&json).unwrap();
                    Ok(Box::new(yak))
                }
                None => Err(redis::RedisError::from((
                    redis::ErrorKind::TypeError,
                    "Yak not found",
                ))),
            }
        }
        Err(err) => {
            tracing::error!("Error: {}", err);
            Err(err)
        }
    }
}

/// Inserts a yak into Redis database.
///
/// # Panics
///
/// Panics if the Redis connection fails.
///
/// # Errors
///
/// This function will return an error if the database query fails.
#[instrument]
pub async fn redis_insert_yak(
    redis: web::Data<redis::Client>,
    yak: Box<Yak>,
) -> Result<Box<Yak>, redis::RedisError> {
    let redis_connection = redis.get_async_connection().await;
    match redis_connection {
        Ok(mut connection) => {
            let yak_json = serde_json::to_string(&yak).unwrap();
            redis::cmd("RPUSH")
                .arg("yak".to_string() + &yak.id().to_string())
                .arg(yak_json)
                .query_async(&mut connection)
                .await?;
            redis::cmd("EXPIRE")
                .arg("yak".to_string() + &yak.id().to_string())
                .arg(60)
                .query_async(&mut connection)
                .await?;
            Ok(yak)
        }
        Err(err) => {
            tracing::error!("Error: {}", err);
            Err(err)
        }
    }
}
