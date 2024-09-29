//! src/startup.rs

use std::net::TcpListener;
use actix_web::dev::Server;
use actix_web::{HttpResponse, HttpServer, App, web};
use crate::routes::{
    greet,
    health_check,
    subscribe,
};

// Notice the different signature!
// We return `Server` on the happy path and we dropped the `async` keyword
// We have no .await call, so it is not needed anymore.
pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    let server = HttpServer::new( || {
        App::new()
            .route("/", web::get().to(greet))
            .route("/health_check", web::get().to(health_check))
            // A new entry in our routing table for POST requests to /subscriptions
            .route("/subscriptions", web::post().to(subscribe))
    })
    .listen(listener)?
    .run();

    Ok(server)
}
