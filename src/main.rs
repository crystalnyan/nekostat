use actix_web::{get, web, App, HttpResponse, HttpServer, Responder, post};

#[get("/")]
async fn home() -> impl Responder {
    HttpResponse::Ok().body("Home")
}

#[get("/hello/{name}")]
async fn greet(name: web::Path<String>) -> impl Responder {
    format!("Hello {name}!")
}

#[post("/translate")]
async fn translate(body: web::Bytes) -> impl Responder {
    let body = String::from_utf8(body.to_vec()).unwrap();
    let neko_text = body.split(" ");
    let mut translated = String::new();
    for word in neko_text {
        translated.push_str(&format!("{}-nya ", word));
    }
    format!("Translated: {translated}")
}

#[actix_web::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/hello", web::get().to(|| async { "Hello World!" }))
            .service(home)
            .service(greet)
            .service(translate)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
