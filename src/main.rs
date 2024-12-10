use chrono::Utc;
use clipboard::{ClipboardContext, ClipboardProvider};
use colored::*;
use scrapers::salesforce::scraper::scrape_salesforce;
use core::panic;
use cron::initialize_cron;
use dialoguer::theme::ColorfulTheme;
use dialoguer::{Confirm, Editor, FuzzySelect, Input, Select};
use discord::initialize_discord_mode;
use dotenv::dotenv;
use handlers::handlers::{
    default_scrape_jobs_handler, handle_craft_a_message, handle_manage_connection,
    handle_open_job_in_browser, handle_reach_out_to_a_connection,
    handle_scan_new_jobs_across_network, prompt_user_for_company_option,
    prompt_user_for_company_selection, prompt_user_for_connection_option,
    prompt_user_for_connection_selection, prompt_user_for_job_option,
    prompt_user_for_job_selection, prompt_user_for_main_menu_selection, CompanyOption,
    FormattedJob, JobOption, MainMenuOption,
};
use handlers::scrape_options::{
    ANDURIL_SCRAPE_OPTIONS, DISCORD_SCRAPE_OPTIONS, GITHUB_SCRAPE_OPTIONS, GITLAB_SCRAPE_OPTIONS,
    ONEPASSWORD_SCRAPE_OPTIONS, PALANTIR_DEFAULT_SCRAPE_OPTIONS,
    THE_BROWSER_COMPANY_DEFAULT_SCRAPE_OPTIONS, TOAST_DEFAULT_SCRAPE_OPTIONS,
    WEEDMAPS_SCRAPE_OPTIONS,
};
use headless_chrome::{Browser, LaunchOptions};
use indicatif::{ProgressBar, ProgressStyle};
use models::data::{AnalyzeData, Company, Connection, Data};
use models::gemini::GeminiJob;
use models::scraper::{Job, JobsPayload};
use reqwest::Client;
use scrapers::blizzard::scraper::scrape_blizzard;
use scrapers::chase::scraper::scrape_chase;
use scrapers::cisco::scraper::scrape_cisco;
use scrapers::coinbase::scraper::scrape_coinbase;
use scrapers::costar_group::scraper::scrape_costar_group;
use scrapers::disney::scraper::scrape_disney;
use scrapers::experian::scraper::scrape_experian;
use scrapers::gen::scraper::scrape_gen;
use scrapers::ibm::scraper::scrape_ibm;
use scrapers::meta::scraper::scrape_meta;
use scrapers::netflix::scraper::scrape_netflix;
use scrapers::reddit::scraper::scrape_reddit;
use scrapers::square::scraper::scrape_square;
use serde_json::{json, Value};
use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::PathBuf;
use std::str::FromStr;
use std::thread::sleep;
use std::time::Duration;
use std::{env, fs};
use strum_macros::{Display, EnumIter};
use tabled::Tabled;
use tabled::{settings::Style, Table};

use tokio::time::Instant;
use tokio_cron_scheduler::{Job as CronJob, JobScheduler};
use utils::clear_console;
use webbrowser;

// TODO: Keys should prob be lowercase, make a tuple where 0 is key and 1 is display name
const COMPANYKEYS: [&str; 22] = [
    "Anduril",
    "Blizzard",
    "Cisco",
    "CoStar Group",
    "Experian",
    "1Password",
    "Weedmaps",
    "Discord",
    "Reddit",
    "GitHub",
    "GitLab",
    "IBM",
    "The Browser Company",
    "Palantir",
    "Coinbase",
    "Gen",
    "Disney",
    "Netflix",
    "Meta",
    "Chase",
    "Square",
    "Salesforce"
];

mod cron;
mod discord;
mod handlers;
mod reports;
mod scrapers;

// mod links
mod utils;
mod models {
    pub mod custom_error;
    pub mod data;
    pub mod gemini;
    pub mod scraper;
}

async fn default_get_job_details(
    job: &Job,
    headless: bool,
    content_selector: &'static str,
) -> Result<GeminiJob, Box<dyn Error>> {
    let launch_options: LaunchOptions = LaunchOptions {
        headless,
        window_size: Some((1920, 1080)),
        ..LaunchOptions::default()
    };

    let browser = Browser::new(launch_options)?;

    let tab = browser.new_tab()?;

    tab.navigate_to(&job.link)?;

    tab.wait_until_navigated()?;

    tab.wait_for_element("body")?;
    let content = tab.wait_for_element(content_selector)?;

    let html = content.get_content()?;

    match GeminiJob::from_job_html(html).await {
        Ok(gemini_job) => Ok(gemini_job),
        Err(e) => {
            eprintln!("Error: {}", e);
            Err(e)
        }
    }
}

/// Hunt for jobs in the terminal
use clap::Parser;

/// Simple CLI application with a cron flag
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Run in cron mode
    #[arg(long, value_name = "HOURS")]
    cron: Option<u64>,

    /// Discord cron mode
    #[arg(long, value_names = &["WEBHOOK_URL", "HOURS"])]
    discord: Option<Vec<String>>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    clear_console();
    dotenv().ok();
    let args = Args::parse();

    // Check if running in cron mode
    if let Some(interval) = args.cron {
        println!("Running in Cron mode!");
        initialize_cron().await?;
        return Ok(());
    }

    if let Some(discord_args) = args.discord {
        if discord_args.len() != 2 {
            eprintln!("Discord integration requires both webhook URL and interval hours");
            std::process::exit(1);
        }

        let webhook_url = discord_args[0].clone();
        let interval: u64 = discord_args[1]
            .as_str()
            .parse()
            .expect("Invalid interval value provide, must be an integer");

        println!("Running in Discord mode!");

        println!("Webhook URL: {webhook_url}");
        println!("Interval: Every {interval} hours");

        initialize_discord_mode(webhook_url, interval).await?;

        return Ok(());
    }

    let dialoguer_styles = ColorfulTheme::default();

    let welcome = figlet_rs::FIGfont::from_file("src/fonts/slant.flf").unwrap();

    let logo = welcome.convert("JobShell").unwrap();

    println!("{logo}");
    sleep(Duration::from_secs(3));

    // INFO: Main App loop
    loop {
        let mut data = Data::get_data();
        clear_console();

        // let counts = data.get_job_counts();
        //
        // println!("{:#?}", counts);
        // sleep(Duration::from_secs(10));

        match prompt_user_for_main_menu_selection() {
            MainMenuOption::ViewBookmarkedJobs => {
                #[derive(Tabled, Debug)]
                struct DisplayJob {
                    company: String,
                    title: String,
                    location: String,
                    link: String,
                }

                loop {
                    clear_console();
                    let (display_jobs, formatted_jobs): (Vec<DisplayJob>, Vec<FormattedJob>) =
                        data.data.iter().fold(
                            (Vec::new(), Vec::new()),
                            |(mut display_jobs, mut formatted_jobs), (company_name, c)| {
                                let (company_dbj, company_bj) = c
                                    .jobs
                                    .iter()
                                    .filter(|j| j.is_bookmarked)
                                    .map(|j| {
                                        (
                                            DisplayJob {
                                                title: j.title.to_string(),
                                                link: j.link.to_string(),
                                                company: company_name.to_string(),
                                                location: j.location.to_string(),
                                            },
                                            FormattedJob {
                                                job: j.clone(),
                                                company: company_name.clone(),
                                                display_name: format!(
                                                    "{} | {} | {}",
                                                    j.title, j.location, company_name
                                                ),
                                            },
                                        )
                                    })
                                    .collect::<(Vec<DisplayJob>, Vec<FormattedJob>)>();

                                display_jobs.extend(company_dbj);
                                formatted_jobs.extend(company_bj);

                                (display_jobs, formatted_jobs)
                            },
                        );

                    let mut table = Table::new(display_jobs);

                    table.with(Style::modern());

                    println!("{table}");
                    let mut titles = formatted_jobs
                        .iter()
                        .map(|job| job.display_name.clone())
                        .collect::<Vec<String>>();

                    titles.push("Exit".to_string());

                    let idx = FuzzySelect::with_theme(&ColorfulTheme::default())
                        .items(&titles)
                        .with_prompt("Select a job")
                        .interact()
                        .unwrap();

                    if titles[idx] == "Exit" {
                        break;
                    }

                    let selected_formatted_job = &formatted_jobs[idx];

                    if let Err(e) = handle_job_option(
                        &selected_formatted_job.job,
                        &mut data,
                        selected_formatted_job.company.as_str(),
                    )
                    .await
                    {
                        eprintln!("Error handling job option: {}", e);
                        // Optional: add a small delay so user can read the error
                        sleep(Duration::from_secs(2));
                    }
                }
            }
            MainMenuOption::SelectACompany => {
                loop {
                    let company_selection = prompt_user_for_company_selection();

                    if company_selection == "Back" {
                        break;
                    }

                    let company = company_selection;

                    //INFO: Company Loop
                    loop {
                        let selected_company_option = prompt_user_for_company_option(company);

                        match selected_company_option {
                            CompanyOption::Back => break,
                            CompanyOption::ViewJobs => {
                                if data.data[company].jobs.is_empty() {
                                    eprintln!("Error: No jobs");
                                    continue;
                                }
                                let jobs = data.data.get(company).unwrap().jobs.clone();
                                //

                                match prompt_user_for_job_selection(jobs, None) {
                                    Some(selected_job) => {
                                        data.mark_job_seen(&selected_job.id);

                                        match handle_job_option(&selected_job, &mut data, company)
                                            .await
                                        {
                                            Ok(()) => {}
                                            Err(e) => eprintln!("Error: {}", e),
                                        }
                                    }
                                    None => break,
                                }
                            }
                            CompanyOption::ViewOrEditConnections => {
                                let company_data = data.data.get(company).unwrap();
                                let connects: Vec<Connection> = company_data.connections.clone();

                                if connects.is_empty() {
                                    println!("You do not have any connections in this company");
                                    continue;
                                }

                                let display_strings: Vec<String> = connects
                                    .iter()
                                    .map(|c| {
                                        format!("{} {} ({})", c.first_name, c.last_name, c.role)
                                    })
                                    .collect();

                                let idx = FuzzySelect::with_theme(&dialoguer_styles)
                                    .with_prompt("Select a connection")
                                    .items(&display_strings)
                                    .interact()
                                    .unwrap();

                                let selected_connection = &connects[idx];

                                let table_style_modern = Style::modern();
                                let mut connection_table = Table::new(vec![selected_connection]);

                                connection_table.with(table_style_modern);

                                println!("{}", connection_table);

                                handle_manage_connection(selected_connection, &mut data, company)
                                    .await?;
                            }
                            // INFO: Scrape Jobs
                            CompanyOption::ScrapeAndUpdateJobs => {
                                let JobsPayload {
                                    all_jobs,
                                    new_jobs,
                                    are_new_jobs,
                                } = match scrape_jobs(&mut data, company).await {
                                    Ok(jp) => jp,
                                    Err(e) => {
                                        let error_string = format!("Error: {}", e).red();
                                        eprintln!("{error_string}");
                                        continue;
                                    }
                                };

                                // TODO: Use 1 FormattedJob struct
                                struct FormattedJob<'a> {
                                    display_string: String,
                                    original_job: &'a Job,
                                }

                                // INFO: Job Selection Loop
                                loop {
                                    match prompt_user_for_job_selection(
                                        all_jobs.clone(),
                                        Some(new_jobs.clone()),
                                    ) {
                                        Some(selected_job) => {
                                            data.mark_job_seen(&selected_job.id);

                                            match handle_job_option(
                                                &selected_job,
                                                &mut data,
                                                company,
                                            )
                                            .await
                                            {
                                                Err(e) => eprintln!("Error: {}", e),
                                                Ok(()) => {}
                                            }
                                        }
                                        None => break,
                                    }
                                }
                            }
                            CompanyOption::AddAConnection => {
                                clear_console();
                                println!("Create a new connection at {}", company);

                                let new_connection =
                                    Connection::create_with_form(&dialoguer_styles, company);

                                if let Some(c) = data.data.get_mut(company) {
                                    let existing_connection = c.connections.iter().find(|&c| {
                                        if c.last_name == new_connection.last_name
                                            && c.first_name == new_connection.first_name
                                        {
                                            return true;
                                        }

                                        return false;
                                    });

                                    if existing_connection.is_some() {
                                        let create_new_connection = Confirm::with_theme(&dialoguer_styles).with_prompt("A connection already exists with the same first name and last name, are you sure you would like to continue creating a new connection?").interact().unwrap();

                                        if !create_new_connection {
                                            continue;
                                        }
                                    }
                                    c.connections.push(new_connection);

                                    data.save();
                                }
                            }
                        }
                    }
                }
            }
            MainMenuOption::ScanForNewJobsAcrossNetwork => {
                match handle_scan_new_jobs_across_network(&mut data).await {
                    Ok(new_jobs) => loop {
                        let options = new_jobs
                            .iter()
                            .map(|fj| fj.display_name.clone())
                            .chain(vec!["Exit".to_string()])
                            .collect::<Vec<String>>();

                        let selection = FuzzySelect::new()
                            .with_prompt("Select a job")
                            .items(&options)
                            .interact()
                            .unwrap();

                        let selected_job = &options[selection];

                        if selected_job == "Exit" {
                            break;
                        }

                        let selected_job = &new_jobs[selection];

                        data.mark_job_seen(&selected_job.job.id);

                        handle_job_option(&selected_job.job, &mut data, &selected_job.company)
                            .await?;
                    },
                    Err(e) => eprintln!("{}", e),
                }
            }
            MainMenuOption::ViewNewJobsReports => handle_view_new_jobs_reports()?,
            MainMenuOption::MyConnections => {
                let all_connections: Vec<&Connection> = data
                    .data
                    .iter()
                    .map(|(_, c)| &c.connections)
                    .flatten()
                    .collect();

                if all_connections.is_empty() {
                    println!("You currently have no connections.");
                    sleep(Duration::from_secs(3));
                    continue;
                }

                let mut table = Table::new(all_connections);
                table.with(Style::modern());

                println!("{}", table);

                Input::<String>::new()
                    .with_prompt("Press enter to continue")
                    .allow_empty(true)
                    .interact()
                    .unwrap();

                continue;
            }
            _ => break,
        }
    }

    Ok(())
}

pub async fn scrape_jobs(
    data: &mut Data,
    company_key: &str,
) -> Result<JobsPayload, Box<dyn Error>> {
    let jobs_payload = match company_key {
        "Anduril" => default_scrape_jobs_handler(data, ANDURIL_SCRAPE_OPTIONS).await,
        "Chase" => scrape_chase(data).await,
        "Cisco" => scrape_cisco(data).await,
        "CoStar Group" => scrape_costar_group(data).await,
        "Blizzard" => scrape_blizzard(data).await,
        "Coinbase" => scrape_coinbase(data).await,
        "Weedmaps" => default_scrape_jobs_handler(data, WEEDMAPS_SCRAPE_OPTIONS).await,
        "1Password" => default_scrape_jobs_handler(data, ONEPASSWORD_SCRAPE_OPTIONS).await,
        "Experian" => scrape_experian(data).await,
        "Discord" => default_scrape_jobs_handler(data, DISCORD_SCRAPE_OPTIONS).await,
        "Palantir" => default_scrape_jobs_handler(data, PALANTIR_DEFAULT_SCRAPE_OPTIONS).await,
        "Reddit" => scrape_reddit(data).await,
        "Gen" => scrape_gen(data).await,
        "IBM" => scrape_ibm(data).await,
        "Disney" => scrape_disney(data).await,
        "Meta" => scrape_meta(data).await,
        "Netflix" => scrape_netflix(data).await,
        "Square" => scrape_square(data).await,
        "Salesforce" => scrape_salesforce(data).await,

        "GitHub" => default_scrape_jobs_handler(data, GITHUB_SCRAPE_OPTIONS).await,
        "GitLab" => default_scrape_jobs_handler(data, GITLAB_SCRAPE_OPTIONS).await,
        "The Browser Company" => {
            default_scrape_jobs_handler(data, THE_BROWSER_COMPANY_DEFAULT_SCRAPE_OPTIONS).await
        }
        "Toast" => default_scrape_jobs_handler(data, TOAST_DEFAULT_SCRAPE_OPTIONS).await,

        _ => return Err(format!("Scraper yet to be implemented for {}", company_key).into()),
    }?;

    Ok(jobs_payload)
}

fn prompt_user_did_apply() -> bool {
    let dialoguer_styles = ColorfulTheme::default();

    let apply = Confirm::with_theme(&dialoguer_styles)
        .with_prompt("Did you apply?")
        .interact()
        .unwrap();

    return apply;
}

// TODO: Use 1 FormattedJob struct

async fn handle_job_option(
    selected_job: &Job,
    data: &mut Data,
    company: &str,
) -> Result<(), Box<dyn Error>> {
    loop {
        let data_job = data.data[company]
            .jobs
            .iter()
            .find(|j| j.id == selected_job.id)
            .unwrap();
        let answer = prompt_user_for_job_option(&data_job);
        match answer {
            JobOption::OpenJobInBrowser => handle_open_job_in_browser(&selected_job, data)?,
            JobOption::ReachOut => {
                handle_reach_out_to_a_connection(&data.data[company].connections, &selected_job)?;
            }
            JobOption::GenerateJobDetails => {
                let job_details = match company {
                    // "Weedmaps" => get_weedmaps_jod_details(&selected_job).await?,
                    "1Password" => default_get_job_details(&selected_job, true, "body").await?,
                    "Tarro" => {
                        default_get_job_details(&selected_job, true, "._content_ud4nd_71").await?
                    }
                    "Discord" => default_get_job_details(&selected_job, true, "body").await?,
                    "Palantir" => default_get_job_details(&selected_job, true, ".content").await?,
                    "Anduril" => default_get_job_details(&selected_job, true, "main").await?,
                    "Coinbase" => {
                        default_get_job_details(
                            &selected_job,
                            false,
                            ".Flex-sc-9cfb0d13-0.Listing__Container-sc-bcedfe82-0.fXHNQM.dBburU",
                        )
                        .await?
                    }
                    _ => default_get_job_details(&selected_job, true, "body").await?,
                };

                // Print details
                // clear_console();
                job_details.print_job();
            }
            JobOption::Bookmark => data.toggle_job_bookmark(&selected_job.id),
            JobOption::Back => break,
        }
    }
    Ok(())
}

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

pub fn handle_view_new_jobs_reports() -> Result<(), Box<dyn Error>> {
    let v = get_new_jobs_report_files();
    let data_path = Data::get_data_dir();

    let reports_path = data_path.join("reports");

    match v {
        Ok(reports) => loop {
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
