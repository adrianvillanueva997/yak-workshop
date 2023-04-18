use sqlx::{postgres::PgPoolOptions, Error, Pool, Postgres};
use tracing::instrument;
/// Creates a connection pool to Postgres.
///
/// # Panics
///
/// Panics if the POSTGRES_URL environment variable is not set.
///
/// # Errors
///
/// This function will return an error if the connection to the database fails.
#[instrument]
pub async fn create_pg_pool() -> Result<Pool<Postgres>, Error> {
    let database_url = std::env::var("POSTGRES_URL").expect("POSTGRES_URL must be set");
    let pg_pool = match PgPoolOptions::new()
        .max_connections(10)
        .connect(&database_url)
        .await
    {
        Ok(pool) => {
            tracing::info!("✅ Connection to Postgres is stablished!");
            Ok(pool)
        }
        Err(err) => {
            tracing::error!("🔥 Failed to connect to Postgres :( {:?}", err);
            Err(err)
        }
    };
    pg_pool
}
