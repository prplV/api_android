mod structs;

use structs::RequestType;
use actix_web::{get, http::StatusCode, post, web, App, HttpResponse, HttpServer, Responder};
use serde::{Serialize, Deserialize};

#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::build(StatusCode::OK)
        .content_type("text/html; charset=utf-8")
        .body("HELLO API")
}

async fn check_crdts(req_body: web::Json<RequestType>) -> impl Responder {
    HttpResponse::Ok()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(index)
            // .service(
            //     web::scope("/api/v1")
            //         .route("/check_credentials", web::post().to(check_crdts))
            // )
    })
        .bind(("127.0.0.1", 11223))?
        .run()
        .await
}