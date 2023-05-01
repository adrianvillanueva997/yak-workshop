use redis::Client;

/// Creates a Redis connection.
///
/// # Panics
///
/// Panics if the REDIS_URL environment variable is not set.
#[tracing::instrument]
pub async fn redis_connection() -> Result<Client, Box<dyn std::error::Error + Send + Sync>> {
    let client = Client::open(std::env::var("REDIS_URL").expect("Redis URL not set"))?;
    match client.get_tokio_connection_manager().await {
        Ok(_) => {
            tracing::info!("✅ Connection to Redis is stablished!");
            Ok(client)
        }
        Err(err) => {
            tracing::error!("Redis connection error: {}", err);
            Err(err.into())
        }
    }
}
