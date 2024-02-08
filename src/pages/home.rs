// src/pages/home.rs
use actix_web::{get, HttpResponse, Responder};

#[get("/")]
pub async fn get_home() -> impl Responder {
    HttpResponse::Ok().body("Welcome to Home Page")
}