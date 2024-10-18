use anyhow::{Ok, Result};
use dotenv::dotenv;
use log::info;
use tokio_postgres::NoTls;
use deadpool_postgres::{Config, ManagerConfig, Pool, RecyclingMethod};
use std::env;
use std::sync::Arc;
use tokio::sync::Mutex;
use deadpool_postgres::Runtime::Tokio1 as tokio_runtime;

pub async fn setup_db_connection() -> Result<Pool> {
    info!("setting up db connection config...");

    let mut cfg = get_pg_config().await?;
    let client = cfg.create_pool(Some(tokio_runtime), NoTls)?;

    Ok(client)
}

async fn get_pg_config() -> Result<Config> {
    let db_host = env::var("NOTES_PG_HOST")?;
    let db_user = env::var("NOTES_PG_USER")?;
    let db_password = env::var("NOTES_PG_PASSWORD")?;
    let db_name = env::var("NOTES_PG_MAIN_DB")?;

    let mut cfg = Config::new();
    cfg.host = Some(db_host);
    cfg.user = Some(db_user);
    cfg.password = Some(db_password);
    cfg.dbname = Some(db_name);
    cfg.manager = Some(ManagerConfig {
        recycling_method : RecyclingMethod::Fast,
    });
    Ok(cfg)
}
