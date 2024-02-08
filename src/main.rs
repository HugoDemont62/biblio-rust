use actix_web::{get, web, App, HttpServer, Responder, HttpResponse};
use serde::Serialize;

#[derive(Serialize)]
struct Book {
    title: String,
    author: String,
}

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body(r#"<a href="/books">Go to books</a>"#)
}

#[get("/echo")]
async fn echo(req_body: String) -> impl Responder {
    req_body
}

#[get("/books")]
async fn get_books() -> impl Responder {
    web::Json(vec![
        Book {
            title: "Moby Dick".to_string(),
            author: "Herman Melville".to_string(),
        },
        Book {
            title: "The Great Gatsby".to_string(),
            author: "F. Scott Fitzgerald".to_string(),
        },
    ])
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Server is starting...");

    let server = HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(echo)
            .service(get_books)
    })
        .bind("127.0.0.1:8080")?;

    let server = server.run();

    std::thread::spawn(|| {
        println!("Server has started.");
    });

    server.await
}