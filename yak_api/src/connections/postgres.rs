use sqlx::{postgres::PgPoolOptions, Error, Pool, Postgres};
pub async fn create_pg_pool() -> Result<Pool<Postgres>, Error> {
    let database_url = std::env::var("POSTGRES_URL").expect("POSTGRES_URL must be set");
    let pg_pool = match PgPoolOptions::new()
        .max_connections(10)
        .connect(&database_url)
        .await
    {
        Ok(pool) => {
            println!("✅Connection to the database is successful!");
            Ok(pool)
        }
        Err(err) => {
            println!("🔥 Failed to connect to the database: {:?}", err);
            Err(err)
        }
    };
    pg_pool
}
