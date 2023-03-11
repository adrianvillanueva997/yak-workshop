use actix_web::{middleware::Logger, web, App, HttpServer};
use sqlx::{postgres::PgPoolOptions, Error, Pool, Postgres};

mod health;
mod metrics;

async fn create_pg_pool() -> Result<Pool<Postgres>, Error> {
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

#[tokio::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    let pg_pool = create_pg_pool().await.unwrap();
    HttpServer::new(move || {
        App::new()
            .service(health::health)
            .service(metrics::metrics)
            .wrap(Logger::default())
            .wrap(Logger::new("%a %t %s %{User-Agent}i"))
            .app_data(web::Data::new(pg_pool.clone()))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
