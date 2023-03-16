use std::net::TcpListener;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    let listener = TcpListener::bind("127.0.0.1:8080").expect("Failed to bind port");
    yak_api::run(listener)?.await
}
