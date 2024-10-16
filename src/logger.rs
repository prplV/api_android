use anyhow::{Ok, Result};
use chrono::Local;
use env_logger::Builder;
use log::{info, LevelFilter};
use std::fs::write;
use std::io::Write;

pub async fn setup_logger() -> Result<()> {
    Builder::new()
        .format(move |buf, record| {
            writeln!(
                buf,
                "API_SERVER: {} [{}] - {}",
                Local::now().format("%d-%m-%Y %H:%M:%S"),
                record.level(),
                record.args(),
            )
        })
        .filter(None, LevelFilter::Info)
        .target(env_logger::Target::Stdout)
        .init();
    info!("logger was successfully created, starting api-server");
    Ok(())
}
