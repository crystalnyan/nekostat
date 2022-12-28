pub mod utils;

use actix_web::{get, web, App, HttpResponse, HttpServer, Responder, post};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/hello", web::get().to(|| async { "Hello World!" }))
            .service(utils::greet)
            .service(utils::translate)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
