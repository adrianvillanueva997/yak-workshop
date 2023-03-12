use actix_web::{middleware::Logger, web, App, HttpServer};
mod health;
mod metrics;
mod postgres;
mod redis;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    let pg_pool = postgres::create_pg_pool().await.unwrap();
    let (redis_client, redis_pool) = redis::redis_connection().await;
    HttpServer::new(move || {
        App::new()
            .service(health::health)
            .service(metrics::metrics)
            .wrap(Logger::default())
            .wrap(Logger::new("%a %t %s %{User-Agent}i"))
            .app_data(web::Data::new(pg_pool.clone()))
            .app_data(web::Data::new(redis_client.clone()))
            .app_data(web::Data::new(redis_pool.clone()))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
