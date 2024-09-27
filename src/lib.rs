//! lib.rs

use actix_web::{web, App, HttpRequest, HttpServer, HttpResponse, Responder};
use actix_web::dev::Server;

async fn greet(req: HttpRequest) -> impl Responder {
    let name = req.match_info().get("name").unwrap_or("World");
    format!("Hello,  {}!", &name)
}

async fn health_check() -> impl Responder {
    HttpResponse::Ok().finish()
}

// Notice the different signature!
// We return `Server` on the happy path and we dropped the `async` keyword
// We have no .await call, so it is not needed anymore.
pub fn run() -> Result<Server, std::io::Error> {
    let server = HttpServer::new( || {
        App::new()
            .route("/", web::get().to(greet))
            .route("/health_check", web::get().to(health_check))
    })
    .bind("127.0.0.1:8000")?
    .run();
    // no longer awaiting here!

    Ok(server)
}
