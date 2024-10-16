use anyhow::{Ok, Result};
use dotenv::dotenv;
use log::info;
use postgres::{Client, NoTls};
use std::env;
use std::sync::Arc;
use tokio::sync::Mutex;

pub async fn setup_db_connection() -> Result<Arc<Mutex<Client>>> {
    info!("setting up db connection config...");
    dotenv()?;

    let db_url = env::var("NOTES_DATABASE_URL")?;

    let client = tokio::task::spawn_blocking(move || {
        let temp = Client::connect(&db_url, NoTls);
        temp
    }).await??;
    // let client = Client::connect(&db_url, NoTls)?;

    Ok(Arc::new(Mutex::new(client)))
}
