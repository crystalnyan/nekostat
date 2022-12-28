use actix_web::{get, web, Responder, post};

#[get("/hello/{name}")]
pub(crate) async fn greet(name: web::Path<String>) -> impl Responder {
    format!("Hello, {name}~nyan!")
}

#[post("/translate")]
pub(crate) async fn translate(body: String) -> impl Responder {
    let neko_text = body.split(" ");
    let mut translated = String::new();
    for word in neko_text {
        translated.push_str(&format!("{}-nya ", word));
    }
    format!("Translated: {translated}")
}