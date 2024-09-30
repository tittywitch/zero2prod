use std::net::TcpListener;
use zero2prod::configuration::get_configuration;
use zero2prod::startup::run;
use sqlx::{PgConnection, Connection, PgPool};

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    // Panic if we can't read config
    let configuration = get_configuration().expect("Failed to read config.");
    let connection_pool = PgPool::connect(
            &configuration.database.connection_string()
        ).await
        .expect("Failed to connect to postgres.");

    // We have removed the hardcoded port 8000
    let address = format!("127.0.0.1:{}", configuration.application_port);

    // Bubble up the io error if we failed to bind the address
    // otherwise call .await on the server
    let listener = TcpListener::bind(address).expect("Failed to bind to port.");

    run(listener, connection_pool)?.await
}
