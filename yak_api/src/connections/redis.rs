use redis::Client;

/// Creates a Redis connection.
///
/// # Panics
///
/// Panics if the REDIS_URL environment variable is not set.
#[tracing::instrument]
pub async fn redis_connection() -> Client {
    let client = Client::open(std::env::var("REDIS_URL").expect("Redis URL not set")).unwrap();
    match client.get_tokio_connection_manager().await {
        Ok(_) => {
            tracing::info!("✅ Connection to Redis is stablished!");
            client
        }
        Err(err) => {
            tracing::error!("Redis connection error: {}", err);
            panic!("Redis connection error: {}", err);
        }
    }
}
