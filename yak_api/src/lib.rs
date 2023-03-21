use std::net::TcpListener;

use actix_web::dev::Server;
use actix_web::web::Data;
use actix_web::{middleware::Logger, web, App, HttpServer};
use routes::{health, metrics, yak};
use sqlx::PgPool;

mod routes;

pub fn run(listener: TcpListener, postgres: PgPool) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(move || {
        App::new()
            .service(
                web::resource("/yak")
                    .name("yak")
                    .route(web::put().to(yak::update_yak))
                    .route(web::delete().to(yak::delete_yak))
                    .route(web::get().to(yak::get_yaks))
                    .route(web::post().to(yak::create_yak)),
            )
            .route("/health", web::get().to(health::health))
            .service(
                web::resource("/metrics")
                    .name("metrics")
                    .route(web::get().to(metrics::metrics)),
            )
            .app_data(Data::new(postgres.clone()))
            .wrap(Logger::default())
            .wrap(Logger::new("%a %t %s %{User-Agent}i"))
    })
    .listen(listener)?
    .run();
    Ok(server)
}
