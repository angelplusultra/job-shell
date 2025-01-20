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

use clap::Parser;
use dialoguer::{theme::ColorfulTheme, FuzzySelect};
use dotenv::dotenv;
use jobshell::utils::clear_console;
use models::data::Data;
use std::{error::Error, fs};

/// Simple CLI application with a cron flag
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Enable Discord mode
    #[arg(long)]
    discord: bool,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
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

// TODO: Refactor this to be a metod on an instance of data or Data
pub fn get_new_jobs_report_files() -> Result<Vec<String>, Box<dyn Error>> {
    let reports_dir = Data::get_data_dir().join("reports");
    let paths = fs::read_dir(reports_dir)?;

    let mut files: Vec<String> = paths
        .filter_map(|entry| {
            let entry = entry.ok()?;
            let path = entry.path();

            // Skip directories, only include files
            if path.is_file() {
                path.file_name()?.to_str().map(|s| s.replace(".html", ""))
            } else {
                None
            }
        })
        .collect();

    files.sort_by(|a, b| b.cmp(a));

    Ok(files)
}

// TODO:  Move this to handlers
pub fn handle_view_new_jobs_reports() -> Result<(), Box<dyn Error>> {
    let v = get_new_jobs_report_files();
    let data_path = Data::get_data_dir();

    let reports_path = data_path.join("reports");

    match v {
        Ok(reports) => loop {
            clear_console();
            let mut options = Vec::from_iter(reports.clone());
            options.push("Back".to_string());

            let idx = FuzzySelect::with_theme(&ColorfulTheme::default())
                .with_prompt("Select a new jobs report")
                .items(&options)
                .interact()
                .unwrap();

            let selected_report = options[idx].as_str();

            if selected_report == "Back" {
                break;
            }

            let report_path = reports_path.join(format!("{}.html", selected_report));
            // Convert the path to a URL format with file:// protocol
            let url = format!("file://{}", report_path.display());

            webbrowser::open(&url)?;
        },
        Err(e) => eprintln!("Error: {}", e),
    }

    Ok(())
}
