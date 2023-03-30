use redis::Client;

#[tracing::instrument]
pub async fn redis_connection() -> Client {
    let client = Client::open(std::env::var("REDIS_URL").expect("Redis URL not set")).unwrap();
    match client.get_tokio_connection_manager().await {
        Ok(_) => {
            tracing::info!("Redis connection established");
            client
        }
        Err(err) => {
            tracing::error!("Redis connection error: {}", err);
            panic!("Redis connection error: {}", err);
        }
    }
}
