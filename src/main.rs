// src/main.rs
mod pages {
    pub mod books;
    pub mod echo;
    pub mod home;
    pub mod add_book; // Assurez-vous d'avoir ce module
}

use actix_web::{App, HttpServer};
use pages::books::get_books;
use pages::echo::echo;
use pages::home::get_home;
use pages::add_book::get_add_book; // Assurez-vous d'avoir cette fonction

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Server is starting...");

    let server = HttpServer::new(|| {
        App::new()
            .service(get_books)
            .service(echo)
            .service(get_home)
            .service(get_add_book) // Assurez-vous d'avoir ce service
    })
        .bind("127.0.0.1:8080")?;

    let server = server.run();

    std::thread::spawn(|| {
        println!("Server has started.");
    });

    server.await
}