//! src/routes/health_check.rs

use actix_web::HttpResponse;

// We were returning `impl Responder` at the beginning.
// We are now declaring the type explicitly given that we have become
// more familiar with actix-web. There is no performance difference.
pub async fn health_check() -> HttpResponse {
    HttpResponse::Ok().finish()
}
