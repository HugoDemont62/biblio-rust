mod pages {
    pub mod books;
    pub mod echo;
}

use actix_web::{App, HttpServer};
use pages::books::get_books;
use pages::echo::echo;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Server is starting...");

    let server = HttpServer::new(|| {
        App::new()
            .service(get_books)
            .service(echo)
    })
        .bind("127.0.0.1:8080")?;

    let server = server.run();

    std::thread::spawn(|| {
        println!("Server has started.");
    });

    server.await
}