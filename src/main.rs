use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};

#[get("/")]
async fn home() -> impl Responder {
    HttpResponse::Ok().body("Home")
}

#[get("/hello/{name}")]
async fn greet(name: web::Path<String>) -> impl Responder {
    format!("Hello {name}!")
}

#[actix_web::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/hello", web::get().to(|| async { "Hello World!" }))
            .service(home)
            .service(greet)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
