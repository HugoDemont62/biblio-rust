use actix_web::{get, HttpResponse, Responder};

#[get("/echo")]
pub async fn echo() -> impl Responder {
    HttpResponse::Ok().body("Echo")
}