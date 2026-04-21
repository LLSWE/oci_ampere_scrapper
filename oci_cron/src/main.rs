use std::sync::Arc;

mod cron;
mod mail;
mod model;
mod script;

use crate::{cron::cron_fn, model::AppConfig};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("STARTING ... OCI CRON");

    let config = Arc::new(AppConfig::load_all());

    cron_fn(config).await?;

    println!("CRON RODANDO ...");

    tokio::signal::ctrl_c().await?;

    println!("GRACEFUL SHUTDOWN...");

    Ok(())
}
