mod args;
mod company_options;
mod discord;
mod error;
mod handlers;
mod modes;
mod reports;
mod scrapers;
mod utils;
mod models {
    pub mod ai;
    pub mod custom_error;
    pub mod data;
    pub mod gemini;
    pub mod scraper;
}

use args::Args;
use clap::Parser;
use dotenv::dotenv;
use error::AppResult;
use jobshell::utils::clear_console;

#[tokio::main]
async fn main() -> AppResult<()> {
    clear_console();
    dotenv().ok();
    let args = Args::parse();

    if args.discord {
        modes::discord::run().await?;
    } else {
        modes::cli::run().await?;
    }

    Ok(())
}
