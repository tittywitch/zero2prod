use zero2prod::run;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    // Bubble up the io error if we failed to bind the address
    // otherwise call .await on the server
    run()?.await
}
