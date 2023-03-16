use std::net::TcpListener;

use actix_web::dev::Server;
use actix_web::{middleware::Logger, web, App, HttpServer};
use routes::{health, metrics};

mod connections;
mod routes;

pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| {
        App::new()
            .route("/health", web::get().to(health::health))
            .service(metrics::metrics)
            .wrap(Logger::default())
            .wrap(Logger::new("%a %t %s %{User-Agent}i"))
    })
    .listen(listener)?
    .run();
    Ok(server)
}
