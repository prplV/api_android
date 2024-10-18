mod db;
mod endpoints;
mod logger;
mod structs;

use actix_web::{
    get, http::StatusCode, post, web, App, HttpRequest, HttpResponse, HttpServer, Responder,
};
use db::setup_db_connection;
use endpoints::{
    check_crdts, index, ping,
    sign_up, create_note, delete_note,
    get_all_notes, check_sqlx
};
use structs::AppState;
use logger::setup_logger;
use serde::{Deserialize, Serialize};

use actix_web::middleware::Logger as Log;

#[actix_web::main]
async fn main() -> anyhow::Result<()> {
    dotenv::dotenv()?;

    setup_logger().await?;
    let db_pool = setup_db_connection().await?;

    anyhow::Result::from(
        HttpServer::new(move || {
            App::new()
                .app_data(web::Data::new(AppState::new(db_pool.clone())))
                .service(index)
                .service(ping).service(
                    web::scope("/api/v1")
                        .route("/check_credentials", web::post().to(check_crdts))
                        .route("/signup", web::post().to(sign_up))
                        .route("/create_note", web::post().to(create_note))
                        .route("/delete_note", web::post().to(delete_note))
                        .route("/get_all_notes", web::post().to(get_all_notes))
                        .route("/check_sqlx", web::post().to(check_sqlx)),
            )
                .wrap(Log::default())
        })
        .bind(("127.0.0.1", 8081))?
        .run()
        .await,
    )?;
    Ok(())
}
