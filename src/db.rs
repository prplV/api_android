use sqlx::{Pool, Postgres, postgres::PgPoolOptions};
use std::env;
use dotenv::dotenv;
use anyhow::{Ok, Result};
use log::info;

pub async fn setup_db_connection() -> Result<()>{
    info!("setting up db connection config...");
    dotenv()?;
    let db_url = env::var("NOTES_DATABASE_URL")?;

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await?;

    Ok(())
}