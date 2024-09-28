//! tests/health_check.rs

use std::net::TcpListener;

// `tokio::test` is the testing equivalent of `tokio::main`
// It also spares you from ahving to specify the `#[test]`  attribute
//
// You can inspect what code gets generated using
// `cargo expand --test health_chheck` (<- name of the test file)
#[tokio::test]
async fn health_check_works() {
    // Arrange
    let address = spawn_app();

    // We need to bring in `reqwest` to perform
    // HTTP requests against our app
    let client = reqwest::Client::new();

    // Act
    let response = client
        .get(&format!("{}/health_check", &address))
        .send()
        .await
        .expect("Failed to execute request");

    // Assert
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}


// Launch our application in the background *somehow*
// No await call therefore no need for spawn app to be async now.
// This is testing so not worth to propagate errors --
// if we fail to perform the required setup we can just panic and crash all the things
fn spawn_app() -> String {
    //   Trying to bind to port 0 will trigger an OS scan for an available port.
    let listener = TcpListener::bind("127.0.0.1:0")
        .expect("Failed to bind to random port");

    // We retrieve the port assigned to us by the OS
    let port = listener.local_addr().unwrap().port();
    let server = zero2prod::run(listener).expect("Failed to bind address");

    // Launch the server as a background task
    // tokio::spawn returns a handle to the spawned future,
    // but we have no use for it here, hence the non binding let
    let _ = tokio::spawn(server);

    // We return the application adress to the caller
    format!("http://127.0.0.1:{}", port)
}
