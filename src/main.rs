use colored::*;
use dialoguer::theme::ColorfulTheme;
use dialoguer::{Confirm, FuzzySelect, Input};
use discord::initialize_discord_mode;
use dotenv::dotenv;
use error::AppResult;
use handlers::handlers::{
    default_scrape_jobs_handler, handle_job_selection, handle_manage_connection,
    handle_manage_smart_criteria, handle_open_job_in_browser, handle_reach_out_to_a_connection,
    handle_scan_new_jobs_across_network_and_followed_companies, prompt_user_for_company_option,
    prompt_user_for_company_selection, prompt_user_for_job_option,
    prompt_user_for_main_menu_selection, CompanyOption, FormattedJob, JobOption, MainMenuOption,
};
use handlers::scrape_options::{
    ANDURIL_SCRAPE_OPTIONS, DISCORD_SCRAPE_OPTIONS, GITHUB_SCRAPE_OPTIONS, GITLAB_SCRAPE_OPTIONS,
    ONEPASSWORD_SCRAPE_OPTIONS, PALANTIR_DEFAULT_SCRAPE_OPTIONS,
    THE_BROWSER_COMPANY_DEFAULT_SCRAPE_OPTIONS, WEEDMAPS_SCRAPE_OPTIONS,
};
use headless_chrome::{Browser, LaunchOptions};
use indicatif::{ProgressBar, ProgressStyle};
use jobshell::utils::clear_console;
use models::data::{Connection, Data};
use models::gemini::GeminiJob;
use models::scraper::{Job, JobsPayload};
use scrapers::airbnb::scraper::scrape_airbnb;
use scrapers::atlassian::scraper::scrape_atlassian;
use scrapers::blizzard::scraper::scrape_blizzard;
use scrapers::chase::scraper::scrape_chase;
use scrapers::cisco::scraper::scrape_cisco;
use scrapers::cloudflare::scraper::scrape_cloudflare;
use scrapers::coinbase::scraper::scrape_coinbase;
use scrapers::costar_group::scraper::scrape_costar_group;
use scrapers::disney::scraper::scrape_disney;
use scrapers::doordash::scraper::scrape_doordash;
use scrapers::experian::scraper::scrape_experian;
use scrapers::gen::scraper::scrape_gen;
use scrapers::ibm::scraper::scrape_ibm;
use scrapers::meta::scraper::scrape_meta;
use scrapers::netflix::scraper::scrape_netflix;
use scrapers::nike::scraper::scrape_nike;
use scrapers::paypal::scraper::scrape_paypal;
use scrapers::reddit::scraper::scrape_reddit;
use scrapers::robinhood::scraper::scrape_robinhood;
use scrapers::salesforce::scraper::scrape_salesforce;
use scrapers::servicenow::scraper::scrape_servicenow;
use scrapers::square::scraper::scrape_square;
use scrapers::stripe::scraper::scrape_stripe;
use scrapers::toast::scraper::scrape_toast;
use scrapers::uber::scraper::scrape_uber;
use std::error::Error;
use std::fs;
use std::thread::sleep;
use std::time::Duration;
use tabled::Tabled;
use tabled::{settings::Style, Table};
use utils::stall_and_present_countdown;
use webbrowser;

// TODO: Keys should prob be lowercase, make a tuple where 0 is key and 1 is display name, or
// straight up just an enum
const COMPANYKEYS: [&str; 33] = [
    "AirBnB",
    "Anduril",
    "Atlassian",
    "Blizzard",
    "Cisco",
    "Cloudflare",
    "CoStar Group",
    "DoorDash",
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
    "Nike",
    "Meta",
    "PayPal",
    "Chase",
    "Robinhood",
    "ServiceNow",
    "Square",
    "Stripe",
    "Salesforce",
    "Toast",
    "Uber",
];

mod cron;
mod discord;
mod handlers;
mod reports;
mod scrapers;

// mod links
mod error;
mod utils;
mod models {
    pub mod ai;
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
        let dialoguer_styles = ColorfulTheme::default();

        let webhook_url = Input::<String>::with_theme(&dialoguer_styles)
            .with_prompt("Enter Discord webhook URL")
            .validate_with(|input: &String| -> Result<(), &str> {
                if input.starts_with("https://discord.com/api/webhooks/") {
                    Ok(())
                } else {
                    Err("Webhook URL must be a valid Discord webhook URL")
                }
            })
            .interact()?;

        let interval = Input::<u64>::with_theme(&dialoguer_styles)
            .with_prompt("Enter scan interval (hours)")
            .validate_with(|input: &u64| -> Result<(), &str> {
                if *input >= 1 && *input <= 12 {
                    Ok(())
                } else {
                    Err("Interval must be between 1 and 12 hours")
                }
            })
            .default(4)
            .interact()?;

        let scan_all_companies = Confirm::with_theme(&dialoguer_styles)
            .with_prompt("Scan all companies? (otherwise only followed companies or companies where you have at least 1 connection)")
            .default(false)
            .interact()?;

        initialize_discord_mode(webhook_url, interval, scan_all_companies)
            .await
            .unwrap_or_else(|e| eprintln!("Error: {}", e));

        return Ok(());
    }

    let dialoguer_styles = ColorfulTheme::default();

    let font_data = include_str!("fonts/slant.flf");
    let welcome =
        figlet_rs::FIGfont::from_content(font_data).expect("Failed to parse embedded font data");

    let logo = welcome
        .convert("JobShell")
        .expect("Failed to convert text to ASCII art");

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
                        data.companies.iter().fold(
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
                    clear_console();
                    let company_selection = prompt_user_for_company_selection();

                    if company_selection == "Back" {
                        break;
                    }

                    let company = company_selection;

                    //INFO: Company Loop
                    loop {
                        clear_console();
                        let is_following = data.companies[company].is_following;
                        let selected_company_option =
                            prompt_user_for_company_option(company, is_following);

                        match selected_company_option {
                            CompanyOption::Back => break,
                            CompanyOption::ViewJobs => {
                                if data.companies[company].jobs.is_empty() {
                                    clear_console();
                                    stall_and_present_countdown(3, Some("No jobs, try scraping"));

                                    continue;
                                }
                                loop {
                                    clear_console();
                                    let jobs = data.companies.get(company).unwrap().jobs.clone();

                                    match handle_job_selection(jobs, None, company) {
                                        Some(selected_job) => {
                                            clear_console();
                                            data.mark_job_seen(&selected_job.id);

                                            match handle_job_option(
                                                &selected_job,
                                                &mut data,
                                                company,
                                            )
                                            .await
                                            {
                                                Ok(()) => {}
                                                Err(e) => eprintln!("Error: {}", e),
                                            }
                                        }
                                        None => break,
                                    }
                                }
                            }
                            CompanyOption::ViewOrEditConnections => {
                                clear_console();
                                let company_data = data.companies.get(company).unwrap();
                                let connects: Vec<Connection> = company_data.connections.clone();

                                if connects.is_empty() {
                                    clear_console();
                                    stall_and_present_countdown(
                                        3,
                                        Some("You do not have any connections in this company"),
                                    );
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
                                clear_console();

                                let spinner = ProgressBar::new_spinner();

                                // Set a custom style (optional)
                                spinner.set_style(
                                    ProgressStyle::default_spinner()
                                        .template("{spinner:.green} {msg}")
                                        .unwrap()
                                        .tick_strings(&["-", "\\", "|", "/"]),
                                );

                                // Set a message
                                spinner.set_message("Loading...");

                                // Start the spinner
                                spinner.enable_steady_tick(Duration::from_millis(120));

                                let JobsPayload {
                                    all_jobs, new_jobs, ..
                                } = match scrape_jobs(&mut data, company).await {
                                    Ok(jp) => jp,
                                    Err(e) => {
                                        let message = e.to_string().red().to_string();
                                        stall_and_present_countdown(3, Some(message));
                                        continue;
                                    }
                                };

                                spinner.finish();
                                // INFO: Job Selection Loop
                                loop {
                                    clear_console();
                                    match handle_job_selection(
                                        all_jobs.clone(),
                                        Some(new_jobs.clone()),
                                        company,
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

                                if let Some(c) = data.companies.get_mut(company) {
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
                            CompanyOption::FollowCompany => {
                                data.toggle_company_follow(company);
                            }
                        }
                    }
                }
            }
            MainMenuOption::ScanForNewJobsAcrossNetworkAndFollowedCompanies => {
                match handle_scan_new_jobs_across_network_and_followed_companies(&mut data).await {
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
                    Err(e) => {
                        stall_and_present_countdown(3, Some(e.to_string()));
                    }
                }
            }
            MainMenuOption::ViewNewJobsReports => handle_view_new_jobs_reports()?,
            MainMenuOption::MyConnections => {
                clear_console();
                let all_connections: Vec<&Connection> = data
                    .companies
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
            MainMenuOption::ManageSmartCriteria => handle_manage_smart_criteria(),
            _ => break,
        }
    }

    Ok(())
}

// TODO: move somewhere
pub async fn scrape_jobs(data: &mut Data, company_key: &str) -> AppResult<JobsPayload> {
    let jobs_payload = match company_key {
        "AirBnB" => scrape_airbnb(data).await,
        "Anduril" => default_scrape_jobs_handler(data, ANDURIL_SCRAPE_OPTIONS).await,
        "Atlassian" => scrape_atlassian(data).await,
        "Chase" => scrape_chase(data).await,
        "Cloudflare" => scrape_cloudflare(data).await,
        "Cisco" => scrape_cisco(data).await,
        "CoStar Group" => scrape_costar_group(data).await,
        "Blizzard" => scrape_blizzard(data).await,
        "Coinbase" => scrape_coinbase(data).await,
        "DoorDash" => scrape_doordash(data).await,
        "Weedmaps" => default_scrape_jobs_handler(data, WEEDMAPS_SCRAPE_OPTIONS).await,
        "1Password" => default_scrape_jobs_handler(data, ONEPASSWORD_SCRAPE_OPTIONS).await,
        "Experian" => scrape_experian(data).await,
        "Discord" => default_scrape_jobs_handler(data, DISCORD_SCRAPE_OPTIONS).await,
        "Palantir" => default_scrape_jobs_handler(data, PALANTIR_DEFAULT_SCRAPE_OPTIONS).await,
        "PayPal" => scrape_paypal(data).await,
        "Reddit" => scrape_reddit(data).await,
        "Robinhood" => scrape_robinhood(data).await,
        "Gen" => scrape_gen(data).await,
        "IBM" => scrape_ibm(data).await,
        "Disney" => scrape_disney(data).await,
        "Meta" => scrape_meta(data).await,
        "Netflix" => scrape_netflix(data).await,
        "Nike" => scrape_nike(data).await,
        "Square" => scrape_square(data).await,
        "Stripe" => scrape_stripe(data).await,
        "Salesforce" => scrape_salesforce(data).await,
        "ServiceNow" => scrape_servicenow(data).await,
        "GitHub" => default_scrape_jobs_handler(data, GITHUB_SCRAPE_OPTIONS).await,
        "GitLab" => default_scrape_jobs_handler(data, GITLAB_SCRAPE_OPTIONS).await,
        "Toast" => scrape_toast(data).await,
        "Uber" => scrape_uber(data).await,
        "The Browser Company" => {
            default_scrape_jobs_handler(data, THE_BROWSER_COMPANY_DEFAULT_SCRAPE_OPTIONS).await
        }

        _ => return Err(format!("Scraper yet to be implemented for {}", company_key).into()),
    }?;

    Ok(jobs_payload)
}

async fn handle_job_option(
    selected_job: &Job,
    data: &mut Data,
    company: &str,
) -> Result<(), Box<dyn Error>> {
    loop {
        clear_console();
        let data_job = data.companies[company]
            .jobs
            .iter()
            .find(|j| j.id == selected_job.id)
            .unwrap();
        let answer = prompt_user_for_job_option(&data_job);
        match answer {
            JobOption::OpenJobInBrowser => handle_open_job_in_browser(&selected_job, data)?,
            JobOption::ReachOut => {
                handle_reach_out_to_a_connection(
                    &data.companies[company].connections,
                    &selected_job,
                )?;
            }
            // JobOption::GenerateJobDetails => {
            //     let job_details = match company {
            //         // "Weedmaps" => get_weedmaps_jod_details(&selected_job).await?,
            //         "1Password" => default_get_job_details(&selected_job, true, "body").await?,
            //         "Tarro" => {
            //             default_get_job_details(&selected_job, true, "._content_ud4nd_71").await?
            //         }
            //         "Discord" => default_get_job_details(&selected_job, true, "body").await?,
            //         "Palantir" => default_get_job_details(&selected_job, true, ".content").await?,
            //         "Anduril" => default_get_job_details(&selected_job, true, "main").await?,
            //         "Coinbase" => {
            //             default_get_job_details(
            //                 &selected_job,
            //                 false,
            //                 ".Flex-sc-9cfb0d13-0.Listing__Container-sc-bcedfe82-0.fXHNQM.dBburU",
            //             )
            //             .await?
            //         }
            //         _ => default_get_job_details(&selected_job, true, "body").await?,
            //     };
            //
            //     // Print details
            //     // clear_console();
            //     job_details.print_job();
            // }
            JobOption::Bookmark => data.toggle_job_bookmark(&selected_job.id),
            JobOption::Back => break,
        }
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
