use actix_web::{get, HttpResponse, Responder};

#[get("/books")]
pub async fn get_books() -> impl Responder {
    HttpResponse::Ok().body("Here are your books")
}