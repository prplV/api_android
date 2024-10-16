mod structs;

use structs::RequestType;
use actix_web::{get, http::StatusCode, post, web, App, HttpResponse, HttpServer, Responder};
use serde::{Serialize, Deserialize};
use dotenv::dotenv;
use sqlx::{Pool, Postgres, postgres::PgPoolOptions};

#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::build(StatusCode::OK)
        .content_type("text/html; charset=utf-8")
        .body("HELLO API")
}
#[get("/ping")]
async fn ping() -> impl Responder {
    HttpResponse::build(StatusCode::OK)
        .content_type("text/html; charset=utf-8")
        .body("PONG")
}

async fn check_crdts(req_body: web::Json<RequestType>) -> impl Responder {
    HttpResponse::Ok()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Starting server on 127.0.0.1:8081");
    HttpServer::new(|| {
        App::new()
            .service(index)
            .service(ping)
            .service(
                web::scope("/api/v1")
                    .route("/check_credentials", web::post().to(check_crdts))
            )
    })
        .bind(("127.0.0.1", 8081))?
        .run()
        .await
}