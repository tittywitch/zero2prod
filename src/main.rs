use zero2prod::run;
use std::net::TcpListener;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    // Bubble up the io error if we failed to bind the address
    // otherwise call .await on the server
    let listener = TcpListener::bind("127.0.0.1:8000")
        .expect("Failed to bind to port.");

    run(listener)?.await
}
