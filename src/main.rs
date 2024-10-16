mod structs;
mod logger;
mod db;

use structs::backend_types::RequestType;
use actix_web::{get, http::StatusCode, post, web, App, HttpRequest, HttpResponse, HttpServer, Responder};
use serde::{Serialize, Deserialize};
use dotenv::dotenv;
use log::{info, error, warn};
use sqlx::{Pool, Postgres, postgres::PgPoolOptions};
use db::setup_db_connection;
use logger::setup_logger;

#[get("/")]
async fn index(req: HttpRequest) -> impl Responder {
    HttpResponse::build(StatusCode::OK)
        .content_type("text/html; charset=utf-8")
        .body("HELLO API")
}
#[get("/ping")]
async fn ping(req: HttpRequest) -> impl Responder {
    HttpResponse::build(StatusCode::OK)
        .content_type("text/html; charset=utf-8")
        .body("PONG")
}

async fn check_crdts(req_body: web::Json<RequestType>) -> impl Responder {
    HttpResponse::Ok()
}

#[actix_web::main]
async fn main() -> anyhow::Result<()> {
    let res = tokio::join!(setup_db_connection(), setup_logger());

    if res.0.is_err() || res.1.is_err() {
        let err =  match res.0 {
            Ok(_) => {
                res.1.unwrap_err()
            },
            Err(er) => er,
        };
        error!("setting up api-server failed due to: {}", err);
    }

    anyhow::Result::from(
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
    )?;
    Ok(())
}