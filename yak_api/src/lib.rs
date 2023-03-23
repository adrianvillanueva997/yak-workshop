use std::net::TcpListener;

use actix_web::dev::{Server, Service};
use actix_web::web::Data;
use actix_web::{middleware::Logger, web, App, HttpServer};
use prometheus::HistogramTimer;
use routes::{health, metrics, yak};
use sqlx::PgPool;
use tracing::instrument;

mod routes;

#[instrument]
pub fn run(listener: TcpListener, postgres: PgPool) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(move || {
        App::new()
            .wrap_fn(|req, srv| {
                let mut histogram_timer: Option<HistogramTimer> = None;
                let request_path = req.path();
                let is_registered_resource = req.resource_map().has_resource(request_path);
                if is_registered_resource {
                    let request_method = req.method().to_string();
                    histogram_timer = Some(
                        metrics::HTTP_RESPONSE_TIME_SECONDS
                            .with_label_values(&[&request_method, request_path])
                            .start_timer(),
                    );
                    metrics::HTTP_REQUESTS_TOTAL
                        .with_label_values(&[&request_method, request_path])
                        .inc();
                }

                let fut = srv.call(req);
                async {
                    let res = fut.await?;
                    if let Some(histogram_timer) = histogram_timer {
                        histogram_timer.observe_duration();
                    };
                    Ok(res)
                }
            })
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
