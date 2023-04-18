use std::net::TcpListener;
mod connections;

/// Main function.
///
/// # Panics
///
/// Panics if the server fails to start or if the Postgres connection fails or if the Redis connection fails.
///
/// # Errors
///
/// This function will return an error if the server fails to start.
#[tokio::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    let listener = TcpListener::bind("127.0.0.1:8080").expect("Failed to bind port");
    let pgpool = connections::postgres::create_pg_pool()
        .await
        .expect("Failed to connect to Postgres");
    let redis_client = connections::redis::redis_connection().await;
    yak_api::run(listener, pgpool, redis_client)?.await
}
