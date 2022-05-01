use serde::Serialize;
use actix_web::{get, App, HttpResponse, HttpServer, Responder};
use actix_web::http::header::{ACCESS_CONTROL_ALLOW_METHODS, ACCESS_CONTROL_ALLOW_ORIGIN};
#[derive(Serialize)]
struct Word {
    id: i32,
    word: String,
    lang: String,
    grammar: String,
    description: String,
}

#[get("/api/words")]
async fn words() -> impl Responder {
    HttpResponse::Ok()
        .insert_header((ACCESS_CONTROL_ALLOW_ORIGIN, "*"))
        .insert_header((ACCESS_CONTROL_ALLOW_METHODS, "GET"))
        .json([
               Word {
                id: 0,
                word: "A".to_string(),
                lang: "A".to_string(),
                grammar: "A".to_string(),
                description: "A".to_string(),
            }, Word {
                id: 1,
                word: "B".to_string(),
                lang: "B".to_string(),
                grammar: "B".to_string(),
                description: "B".to_string(),
        }])
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(words)
    })
        .bind(("127.0.0.1", 8000))?
        .run()
        .await
}