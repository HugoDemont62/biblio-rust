use actix_web::{get, App, HttpServer, Responder};

#[get("/")]
async fn hello() -> impl Responder {
    "Hello World!"
}

#[get("/echo")]
async fn echo(req_body: String) -> impl Responder {
    req_body
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Server is starting...");

    let server = HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(echo)
    })
        .bind("127.0.0.1:8080")?;

    let server = server.run();

    std::thread::spawn(|| {
        println!("Server has started.");
    });

    server.await
}