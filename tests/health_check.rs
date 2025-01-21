use std::net::TcpListener;

use z2p::run;

#[actix_web::test]
async fn health_check_works() {
    let address = spawn_app().await;

    let client = reqwest::Client::new();

    let response = client
        .get(format!("{address}/health_check"))
        .send()
        .await
        .expect("Failed to execute request");

    assert!(response.status().is_success());
    assert_eq!(response.content_length(), Some(0));
}

async fn spawn_app() -> String {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind address.");
    let local_addr = listener.local_addr().expect("Failed to get address.");
    let app = run(listener).expect("Failed to run app.");
    actix_web::rt::spawn(app);
    format!("http://{}", local_addr)
}
