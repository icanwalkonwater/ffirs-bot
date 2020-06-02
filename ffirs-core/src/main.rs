use anyhow::Result;
use dotenv_codegen::dotenv;
use ffirs_core::{event_handler::Handler, LOG_LEVEL};
use log::warn;
use serenity::Client;
use simplelog::{Config, SimpleLogger, TermLogger, TerminalMode};

#[tokio::main]
async fn main() -> Result<()> {
    dotenv::dotenv()?;

    TermLogger::init(LOG_LEVEL, Config::default(), TerminalMode::Mixed).unwrap_or_else(|_| {
        SimpleLogger::init(LOG_LEVEL, Config::default()).expect("Failed to setup a logger !");
        warn!("Failed to setup TermLogger, using SimpleLogger.");
    });

    let mut client = Client::new(dotenv!("TOKEN")).event_handler(Handler).await?;

    #[cfg(not(feature = "sharded"))]
    client.start().await?;
    #[cfg(feature = "sharded")]
    client.start_autosharded().await?;

    Ok(())
}
