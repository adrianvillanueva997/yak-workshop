use redis::{aio::ConnectionManager, Client};

#[tracing::instrument]
pub async fn redis_connection() -> (Client, ConnectionManager) {
    let client = Client::open(std::env::var("REDIS_URL").expect("Redis URL not set")).unwrap();
    match client.get_tokio_connection_manager().await {
        Ok(con) => {
            tracing::info!("Redis connection established");
            (client, con)
        }
        Err(err) => {
            tracing::error!("Redis connection error: {}", err);
            panic!("Redis connection error: {}", err);
        }
    }
}
