use redis::{aio::ConnectionManager, Client};

pub async fn redis_connection() -> (Client, ConnectionManager) {
    let client = Client::open(std::env::var("REDIS_URL").expect("Redis URL not set")).unwrap();
    let con = client.get_tokio_connection_manager().await.unwrap();
    println!("Redis connection established");
    (client, con)
}
