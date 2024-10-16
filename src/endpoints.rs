use crate::structs::backend_types::RequestType;
use actix_web::http::StatusCode;
use actix_web::{get, web, HttpRequest, HttpResponse, Responder};
use crate::structs::AppState;

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

// POST : check credentials (JSON-IN) (login) -> empty packet with status
// POST : sign up (JSON-IN) -> JSON
// POST : create note (JSON-IN) -> empty packet with status
// POST : delete note (JSON-IN) -> empty packet with status
// POST : get all notes (JSON-IN) -> JSON
pub async fn check_crdts(pool: web::Data<AppState>, req_body: web::Json<RequestType>) -> impl Responder {
    HttpResponse::Ok()
}
pub async fn sign_up(pool: web::Data<AppState>, req_body: web::Json<RequestType>) -> impl Responder {
    HttpResponse::Ok()
}
pub async fn create_note(pool: web::Data<AppState>, req_body: web::Json<RequestType>) -> impl Responder {
    HttpResponse::Ok()
}
pub async fn delete_note(pool: web::Data<AppState>, req_body: web::Json<RequestType>) -> impl Responder {
    HttpResponse::Ok()
}
pub async fn get_all_notes(req_body: web::Json<RequestType>) -> impl Responder {
    HttpResponse::Ok()
}
pub async fn check_sqlx(pool: web::Data<AppState>, req_body: HttpRequest) -> impl Responder {
    // let client = pool.db_pool.clone();
    if let Ok(row) = pool.db_pool.clone().lock().await.query("select * from users", &[]) {
        println!("{:?}", row);
    }
    HttpResponse::Ok()
}
// pub async fn