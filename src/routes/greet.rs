//! src/routes/greet.rs

use actix_web::{Responder, HttpRequest};

pub async fn greet(req: HttpRequest) -> impl Responder {
    let name = req.match_info().get("name").unwrap_or("World");
    format!("Hello,  {}!", &name)
}

