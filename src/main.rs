use actix_web::{post, web, App, HttpRequest, HttpServer, Responder};
use serde::Deserialize;

#[derive(Deserialize)]
struct FormData {
    name: String,
    age: String,
}

#[post("*")]
async fn sms(req: HttpRequest, form: web::Form<FormData>) -> impl Responder {
    let url = req.uri();
    format!("Hello {}:{} {}!", form.name, form.age, url)
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(sms))
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
