// src/pages/home.rs
use actix_web::{get, HttpResponse, Responder};

#[get("/")]
pub async fn get_home() -> impl Responder {
    HttpResponse::Ok().body("
    <html>
    <head>
    <title>Home</title>
    </head>
    <body>
    <h1>Bienvenue</h1>
    <p>Ceci est la page d'accueil</p>
    <a href='/add-book'><button>Ajouter un livre</button></a>
    <a href='/books'><button>Voir tous les livres</button></a>
    </body>
    </html>
    ")
}