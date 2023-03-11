use actix_web::{App, HttpServer};
mod health;
mod metrics;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(health::health).service(metrics::metrics))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
