use actix_web::http::header;
use actix_web::HttpResponse;
use lazy_static::lazy_static;
use prometheus::{
    opts, register_histogram_vec, register_int_counter_vec, register_int_gauge, Encoder,
    HistogramVec, IntCounterVec, IntGauge, TextEncoder,
};

const HTTP_RESPONSE_TIME_CUSTOM_BUCKETS: &[f64; 14] = &[
    0.0005, 0.0008, 0.00085, 0.0009, 0.00095, 0.001, 0.00105, 0.0011, 0.00115, 0.0012, 0.0015,
    0.002, 0.003, 1.0,
];

lazy_static! {
    pub static ref HTTP_REQUESTS_TOTAL: IntCounterVec = register_int_counter_vec!(
        opts!("http_requests_total", "HTTP requests total"),
        &["method", "path"]
    )
    .expect("Can't create a metric");
    pub static ref HTTP_CONNECTED_SSE_CLIENTS: IntGauge =
        register_int_gauge!(opts!("http_connected_sse_clients", "Connected SSE clients"))
            .expect("Can't create a metric");
    pub static ref HTTP_RESPONSE_TIME_SECONDS: HistogramVec = register_histogram_vec!(
        "http_response_time_seconds",
        "HTTP response times",
        &["method", "path"],
        HTTP_RESPONSE_TIME_CUSTOM_BUCKETS.to_vec()
    )
    .expect("Can't create a metric");
}

/// Get metrics from Prometheus.
///
/// # Panics
///
/// Panics if the metrics can't be encoded.
pub async fn metrics() -> HttpResponse {
    let encoder = TextEncoder::new();
    let mut buffer = vec![];
    encoder
        .encode(&prometheus::gather(), &mut buffer)
        .expect("Failed to encode metrics");

    let response = String::from_utf8(buffer.clone()).expect("Failed to convert bytes to string");
    buffer.clear();

    HttpResponse::Ok()
        .insert_header(header::ContentType(mime::TEXT_PLAIN))
        .body(response)
}
