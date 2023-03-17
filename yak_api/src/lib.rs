use std::net::TcpListener;

use actix_web::dev::Server;
use actix_web::{middleware::Logger, web, App, HttpServer};
use routes::{health, metrics, yak};

mod connections;
mod routes;

pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| {
        App::new()
            .service(
                web::resource("/yak")
                    .name("name")
                    .route(web::get().to(yak::index)),
            )
            .route("/health", web::get().to(health::health))
            .service(
                web::resource("/metrics")
                    .name("metrics")
                    .route(web::get().to(metrics::metrics)),
            )
            .wrap(Logger::default())
            .wrap(Logger::new("%a %t %s %{User-Agent}i"))
    })
    .listen(listener)?
    .run();
    Ok(server)
}
