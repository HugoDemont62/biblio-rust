// src/pages/add_book.rs
use actix_web::{get, HttpResponse, Responder};

#[get("/add-book")]
pub async fn get_add_book() -> impl Responder {
    HttpResponse::Ok().body("
    <html>
    <head>
    <title>Ajouter un livre</title>
    </head>
    <body>
    <h1>Ajouter un livre</h1>
    <form>
    <label for='title'>Titre:</label><br>
    <input type='text' id='title' name='title'><br>
    <label for='author'>Auteur:</label><br>
    <input type='text' id='author' name='author'><br>
    <label for='year'>Année de publication:</label><br>
    <input type='number' id='year' name='year'><br>
    <label for='genre'>Genre:</label><br>
    <input type='text' id='genre' name='genre'><br>
    <label for='publisher'>Éditeur:</label><br>
    <input type='text' id='publisher' name='publisher'><br>
    <label for='notes'>Notes:</label><br>
    <textarea id='notes' name='notes'></textarea><br>
    <input type='submit' value='Ajouter le livre'>
    </form>
    </body>
    </html>
    ")
}