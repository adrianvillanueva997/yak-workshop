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
        QueryBuilder::new("SELECT id,name,age, age_last_shaved from yak WHERE id = $1");
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
#[instrument]
pub async fn redis_fetch_all_yaks(
    redis: web::Data<redis::Client>,
) -> Result<Vec<Yak>, redis::RedisError> {
    let redis_connection = redis.get_async_connection().await;
    match redis_connection {
        Ok(mut connection) => {
            let yaks: Vec<Yak> = redis::cmd("LRANGE")
                .arg("yaks")
                .arg(0)
                .arg(-1)
                .query_async(&mut connection)
                .await?;
            Ok(yaks)
        }
        Err(err) => {
            tracing::error!("Error: {}", err);
            Err(err)
        }
    }
}
#[instrument]
pub async fn redis_insert_all_yaks(
    redis: web::Data<redis::Client>,
    yaks: Vec<Yak>,
) -> Result<(), redis::RedisError> {
    let redis_connection = redis.get_async_connection().await;
    match redis_connection {
        Ok(mut connection) => {
            for yak in yaks {
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
#[instrument]
pub async fn redis_fetch_yak(
    redis: web::Data<redis::Client>,
    id: i32,
) -> Result<Yak, redis::RedisError> {
    let redis_connection = redis.get_async_connection().await;
    match redis_connection {
        Ok(mut connection) => {
            let yaks: Vec<Yak> = redis::cmd("LRANGE")
                .arg("yaks")
                .arg(0)
                .arg(-1)
                .query_async(&mut connection)
                .await?;
            let yak = yaks.iter().find(|y| y.id() == id);
            match yak {
                Some(yak) => Ok(yak.clone()),
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

#[instrument]
pub async fn redis_insert_yak(
    redis: web::Data<redis::Client>,
    yak: Yak,
) -> Result<Yak, redis::RedisError> {
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
