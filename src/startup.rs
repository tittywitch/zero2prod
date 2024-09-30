//! src/startup.rs

use crate::routes::{greet, health_check, subscribe};
use actix_web::dev::Server;
use actix_web::{web, App, HttpResponse, HttpServer};
use std::net::TcpListener;
use sqlx::{PgConnection, PgPool};

// Notice the different signature!
// We return `Server` on the happy path and we dropped the `async` keyword
// We have no .await call, so it is not needed anymore.
pub fn run(listener: TcpListener, db_pool: PgPool) -> Result<Server, std::io::Error> {
    // wrap the pool using web::Data, which boils down to an Arc smart pointer
    let db_pool = web::Data::new(db_pool);
    // capture connection from the surrounding environment
    let server = HttpServer::new(move || {
        App::new()
            .route("/", web::get().to(greet))
            .route("/health_check", web::get().to(health_check))
            // A new entry in our routing table for POST requests to /subscriptions
            .route("/subscriptions", web::post().to(subscribe))
            // Register the connection as part of the application state of actix-web
            .app_data(db_pool.clone())
    })
    .listen(listener)?
    .run();

    Ok(server)
}
