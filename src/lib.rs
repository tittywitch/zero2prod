//! lib.rs

use actix_web::{web, App, HttpRequest, HttpServer, HttpResponse, Responder};
use actix_web::dev::Server;
use std::net::TcpListener;

async fn greet(req: HttpRequest) -> impl Responder {
    let name = req.match_info().get("name").unwrap_or("World");
    format!("Hello,  {}!", &name)
}

// We were returning `impl Responder` at the beginning.
// We are now declaring the type explicitly given that we have become
// more familiar with actix-web. There is no performance difference.
async fn health_check() -> HttpResponse {
    HttpResponse::Ok().finish()
}


#[derive(serde::Deserialize)]
struct FormData {
    email: String,
    name:  String,
}

// Let's start simple -- we always return a 200 OK
async fn subscribe(_form: web::Form<FormData>) -> HttpResponse {
    HttpResponse::Ok().finish()
}

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
