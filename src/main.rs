mod pages {
    pub mod books;
    pub mod echo;
    pub mod home;
}

use actix_web::{App, HttpServer};
use pages::books::get_books;
use pages::echo::echo;
use pages::home::get_home;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Server is starting...");

    let server = HttpServer::new(|| {
        App::new()
            .service(get_books)
            .service(echo)
            .service(get_home)
    })
        .bind("127.0.0.1:8080")?;

    let server = server.run();

    std::thread::spawn(|| {
        println!("Server has started.");
    });

    server.await
}