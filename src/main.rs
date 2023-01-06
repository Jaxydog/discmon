#![feature(const_trait_impl, is_some_and, iter_array_chunks, result_flattening)]
#![deny(clippy::expect_used, clippy::panic, clippy::unwrap_used)]
#![warn(clippy::cargo, clippy::nursery, clippy::pedantic)]
#![warn(clippy::todo, clippy::unimplemented, clippy::unreachable)]
#![allow(clippy::multiple_crate_versions, clippy::unused_async)]
#![allow(clippy::module_name_repetitions)]

use clap::Parser;
use prelude::*;

mod command;
mod event;
mod prelude;
mod util;

pub const DEV_BUILD: bool = cfg!(debug_assertions);
pub const INTENTS: GatewayIntents = GatewayIntents::non_privileged();

/// Pokemon rolling bot for fun and not really profit but y'know
#[derive(Debug, Parser)]
#[command(author, about, version)]
struct Args {
    /// Disables the logger's console output
    #[arg(long, short)]
    quiet: bool,
    /// Disables the logger's file output
    #[arg(long, short)]
    ephemeral: bool,
    /// The number of seconds between clock ticks
    #[arg(default_value = "10", long, short)]
    clock: u64,
}

fn token() -> Result<String> {
    std::env::var(if DEV_BUILD { "DEV_TOKEN" } else { "TOKEN" }).map_err(Into::into)
}
fn dev_guild() -> Result<GuildId> {
    Ok(GuildId::new(std::env::var("DEV_GUILD")?.parse()?))
}

async fn clock(logger: Logger, secs: u64, token: String) -> ! {
    let mut interval = tokio::time::interval(std::time::Duration::from_secs(secs));
    let http = std::sync::Arc::new(Http::new(&token));

    info!(logger, "Clock started ({secs} secs)");

    loop {
        interval.tick().await;
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    dotenvy::dotenv()?;
    let Args {
        quiet,
        ephemeral,
        clock: secs,
    } = Args::try_parse()?;

    let token = token()?;
    let logger = Logger::new(quiet, ephemeral)?;

    info!(logger, "Starting...");

    let event_handler = Events::new(logger.clone());
    let mut client = Client::builder(&token, INTENTS)
        .event_handler(event_handler)
        .await?;

    tokio::spawn(clock(logger, secs, token.clone()));
    client.start_autosharded().await.map_err(Into::into)
}
