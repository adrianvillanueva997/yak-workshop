// #[cfg(test)]
// mod tests {
//     use std::net::TcpListener;

//     fn spawn_server() -> String {
//         let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");
//         let port = listener.local_addr().unwrap().port();
//         let server = yak_api::run(listener).expect("Failed to bind address");
//         let _ = tokio::spawn(server);
//         format!("http://127.0.0.1:{}", port)
//     }

//     #[tokio::test]
//     async fn test_health() {
//         let address = spawn_server();
//         let client = reqwest::Client::new();
//         let response = client
//             .get(&format!("{}/health", address))
//             .send()
//             .await
//             .expect("Failed to execute request");
//         assert_eq!(response.status(), 200);
//         assert_eq!(Some(0), response.content_length());
//     }
// }
