use actix_web::{get, HttpResponse, Responder};
use serde_json::json;

#[get("/healthchecker")]
pub async fn healthchecker() -> impl Responder {
    HttpResponse::Ok().json(json!({
        "status": "success",
        "message": "Hello from Rust!",
    }))
}