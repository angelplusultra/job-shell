use clipboard::{ClipboardContext, ClipboardProvider};
use colored::*;
use core::panic;
use dialoguer::theme::ColorfulTheme;
use dialoguer::{Confirm, Editor, FuzzySelect, Input, Select};
use dotenv::dotenv;
use handlers::handlers::default_scrape_jobs_handler;
use handlers::scrape_options::{
    ANDURIL_SCRAPE_OPTIONS, DISCORD_SCRAPE_OPTIONS, GITHUB_SCRAPE_OPTIONS, GITLAB_SCRAPE_OPTIONS,
    ONEPASSWORD_SCRAPE_OPTIONS, PALANTIR_DEFAULT_SCRAPE_OPTIONS,
    THE_BROWSER_COMPANY_DEFAULT_SCRAPE_OPTIONS, TOAST_DEFAULT_SCRAPE_OPTIONS,
    WEEDMAPS_SCRAPE_OPTIONS,
};
use headless_chrome::{Browser, LaunchOptions};
use indicatif::{ProgressBar, ProgressStyle};
use models::data::{Company, Connection, Data};
use models::gemini::GeminiJob;
use models::scraper::{Job, JobsPayload};
use scrapers::blizzard::scraper::scrape_blizzard;
use scrapers::coinbase::scraper::scrape_coinbase;
use scrapers::reddit::scraper::scrape_reddit;
use std::collections::{HashMap, HashSet};
use std::env;
use std::error::Error;
use std::io::Write;
use std::thread::sleep;
use std::time::Duration;
use tabled::Table;
use tokio::time::Instant;
use webbrowser;

// TODO: Keys should prob be lowercase, make a tuple where 0 is key and 1 is display name
const COMPANYKEYS: [&str; 11] = [
    "Anduril",
    "1Password",
    "Weedmaps",
    "Discord",
    "Reddit",
    "GitHub",
    "GitLab",
    "The Browser Company",
    "Palantir",
    "Coinbase", // (In Development)
    "Toast",
    // "Blizzard" (In Development),
];
mod handlers;
mod scrapers;

// mod links
mod utils {
    pub mod stringify_js;
}
mod models {
    pub mod custom_error;
    pub mod data;
    pub mod gemini;
    pub mod scraper;
}

fn clear_console() {
    print!("\x1B[2J\x1B[1;1H");
    std::io::stdout().flush().unwrap()
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

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    clear_console();
    dotenv().ok();

    let dialoguer_styles = ColorfulTheme::default();

    let welcome = figlet_rs::FIGfont::from_file("src/fonts/slant.flf").unwrap();

    let logo = welcome.convert("JobShell").unwrap();

    println!("{logo}");
    sleep(Duration::from_secs(3));

    // INFO: Main App loop
    loop {
        clear_console();

        let mut data = Data::get_data();

        let main_menu_options = [
            "Select a Company",
            "Network-Wide Scrape",
            "My Connections",
            "Exit",
        ];

        let main_menu_selection = main_menu_options[Select::with_theme(&dialoguer_styles)
            .with_prompt("Select an option")
            .items(&main_menu_options)
            .interact()
            .unwrap()];

        match main_menu_selection {
            "Select a Company" => {
                //TODO: Scrape a specific company, manage connections at a specifc company
                let mut company_options = COMPANYKEYS.to_vec();

                company_options.sort();

                // company_options.push("Scrape My Connection Jobs");
                // company_options.push("View All Connections");
                company_options.push("Back");

                // INFO: Prompt User for Company Selection
                let selection = FuzzySelect::with_theme(&dialoguer_styles)
                    .with_prompt("What do you choose?")
                    .items(&company_options)
                    .interact()
                    .unwrap();

                let company_selection = company_options[selection];

                if company_selection == "Back" {
                    continue;
                }

                let company = company_selection;

                let options = [
                    "Scrape Jobs",
                    "Add a Connection",
                    "View/Edit Connections",
                    "Back",
                ];

                //INFO: Company Loop
                loop {
                    let selection = Select::with_theme(&dialoguer_styles)
                        .with_prompt(&format!("Select an option for {}", company))
                        .items(&options)
                        .interact()
                        .unwrap();

                    match options[selection] {
                        "Back" => break,
                        "View/Edit Connections" => {
                            let connects = &data.data[company].connections;

                            if connects.is_empty() {
                                println!("You do not have any connections in this company");
                                continue;
                            }

                            let display_strings: Vec<String> = connects
                                .iter()
                                .map(|c| format!("{} {} ({})", c.first_name, c.last_name, c.role))
                                .collect();

                            let idx = FuzzySelect::with_theme(&dialoguer_styles)
                                .with_prompt("Select a connection")
                                .items(&display_strings)
                                .interact()
                                .unwrap();

                            let selected_connection = &connects[idx];

                            let connection_table = Table::new(vec![selected_connection]);

                            println!("{}", connection_table);

                            let connections_options = ["Edit", "Open LinkedIn", "Delete", "Back"];
                            let selected_connection_option = connections_options
                                [Select::with_theme(&dialoguer_styles)
                                    .with_prompt("Select an option")
                                    .items(&connections_options)
                                    .interact()
                                    .unwrap()];

                            match selected_connection_option {
                                "Edit" => todo!(),
                                "Open LinkedIn" => todo!(),
                                "Delete" => todo!(),
                                "Back" => continue,
                                _ => panic!(),
                            }
                        }
                        // INFO: Scrape Jobs
                        "Scrape Jobs" => {
                            let JobsPayload {
                                all_jobs,
                                new_jobs,
                                are_new_jobs,
                            } = scrape_jobs(&mut data, company).await?;

                            struct FormattedJob<'a> {
                                display_string: String,
                                original_job: &'a Job,
                            }

                            // INFO: Job Selection Loop
                            loop {
                                // INFO: Format jobs for presentation
                                //
                                let jobs = data.data[company].jobs.clone();

                                let mut formatted_options = jobs
                                    .iter()
                                    .map(|j| {
                                        let mut display_string =
                                            format!("🧳 {} | 🌎 {}", j.title, j.location);

                                        let new_job = new_jobs.iter().find(|nj| j.id == nj.id);
                                        if !j.is_seen {
                                            display_string += " 👀"
                                                .bright_green()
                                                .bold()
                                                .to_string()
                                                .as_str();
                                        }

                                        if new_job.is_some() {
                                            display_string += " ❗"
                                                .bright_green()
                                                .bold()
                                                .to_string()
                                                .as_str();
                                        }

                                        FormattedJob {
                                            display_string,
                                            original_job: j,
                                        }
                                    })
                                    .collect::<Vec<FormattedJob>>();

                                // INFO: Filter jobs down by locations if data set too large
                                if all_jobs.len() > 99 {
                                    let locations = all_jobs
                                        .iter()
                                        .fold(HashSet::new(), |mut hash, job| {
                                            hash.insert(&job.location);

                                            return hash;
                                        })
                                        .into_iter()
                                        .collect::<Vec<&String>>();

                                    let selection = Select::with_theme(&dialoguer_styles)
                                        .with_prompt("Select location")
                                        .items(&locations)
                                        .interact()
                                        .unwrap();

                                    let selected_location = locations[selection];

                                    formatted_options
                                        .retain(|j| &j.original_job.location == selected_location);
                                }

                                let mut display_options = formatted_options
                                    .iter()
                                    .map(|j| j.display_string.as_str())
                                    .collect::<Vec<&str>>();

                                let prompt = format!(
                                    "Select a job ({}, 👀: Unseen, ❗: New Listing)",
                                    display_options.len()
                                );
                                // Pushing Exit option
                                display_options.push("Exit");

                                // Prompt user for job selection
                                let selected_job = FuzzySelect::new()
                                    .with_prompt(&prompt)
                                    .items(&display_options)
                                    .interact()
                                    .unwrap();

                                if display_options[selected_job] == "Exit" {
                                    break;
                                }

                                let job = formatted_options[selected_job].original_job;
                                // INFO: Mark Job as seen
                                data.data
                                    .get_mut(company)
                                    .unwrap()
                                    .jobs
                                    .iter_mut()
                                    .find(|j| j.id == job.id)
                                    .unwrap()
                                    .is_seen = true;

                                data.save();
                                loop {
                                    let prompt = format!("Select an option for {}", job.title);

                                    let job_options = [
                                        "Open Job in Browser",
                                        "Reach Out to a Connection",
                                        "Generate Job Details with AI",
                                        "Back",
                                    ];
                                    let job_options_selection =
                                        job_options[Select::with_theme(&dialoguer_styles)
                                            .with_prompt(prompt)
                                            .items(&job_options)
                                            .interact()
                                            .unwrap()];

                                    match job_options_selection {
                                        "Open Job in Browser" => {
                                            webbrowser::open(&job.link)?;

                                            let apply = Confirm::with_theme(&dialoguer_styles)
                                                .with_prompt("Did you apply?")
                                                .interact()
                                                .unwrap();

                                            if apply {
                                                if let Some(company) = data.data.get_mut(company) {
                                                    //TODO: search by ID field when added to struct
                                                    let selected_job = company
                                                        .jobs
                                                        .iter_mut()
                                                        .find(|j| j.id == job.id)
                                                        .unwrap();
                                                    selected_job.applied = true;

                                                    data.save();
                                                }
                                            }

                                            continue;
                                        }
                                        "Reach Out to a Connection" => {
                                            let connections = &data.data[company].connections;

                                            // TODO: If connections is empty, print message and continue loop

                                            let display_strings: Vec<String> = connections
                                                .iter()
                                                .map(|c| {
                                                    format!(
                                                        "{} {} ({})",
                                                        c.first_name, c.last_name, c.role
                                                    )
                                                })
                                                .collect();

                                            let idx = Select::with_theme(&dialoguer_styles)
                                                .with_prompt("Select a connection")
                                                .items(&display_strings)
                                                .interact()
                                                .unwrap();

                                            let selected_connection = &connections[idx];

                                            let connection_table =
                                                Table::new(vec![selected_connection]);

                                            println!("{}", connection_table);

                                            let mut connections_options =
                                                vec!["Craft a Message", "Back"];

                                            if selected_connection.linkedin.is_some() {
                                                connections_options.insert(0, "Open LinkedIn")
                                            }

                                            let selected_connection_option = connections_options
                                                [Select::with_theme(&dialoguer_styles)
                                                    .with_prompt("Select an option")
                                                    .items(&connections_options)
                                                    .interact()
                                                    .unwrap()];

                                            match selected_connection_option {
                                                "Open LinkedIn" => {
                                                    let linkedin_url = selected_connection
                                                        .linkedin
                                                        .as_ref()
                                                        .unwrap();

                                                    webbrowser::open(linkedin_url)?;
                                                }
                                                "Craft a Message" => {
                                                    let mut message = Editor::new()
                                                        .edit("Craft your message")
                                                        .unwrap()
                                                        .unwrap();

                                                    message += &format!("\n\n{}", job.link);
                                                    let mut clipboard: ClipboardContext =
                                                        ClipboardProvider::new().unwrap();
                                                    clipboard
                                                        .set_contents(message.clone())
                                                        .unwrap();

                                                    println!("Your message has been copied to your clipboard.");

                                                    if selected_connection.linkedin.is_some() {
                                                        let open_linkedIn =
                                                            Confirm::with_theme(&dialoguer_styles)
                                                                .with_prompt("Open LinkedIn?")
                                                                .interact()
                                                                .unwrap();
                                                        if open_linkedIn {
                                                            webbrowser::open(
                                                                &selected_connection
                                                                    .linkedin
                                                                    .as_ref()
                                                                    .unwrap(),
                                                            );
                                                        }
                                                    }
                                                }

                                                "Back" => break,
                                                _ => panic!(),
                                            }
                                        }
                                        "Generate Job Details with AI" => {
                                            let job_details = match company {
                            // "Weedmaps" => get_weedmaps_jod_details(&job).await?,
                            "1Password" => default_get_job_details(&job, true, "body").await?,
                            "Tarro" => {
                                default_get_job_details(&job, true, "._content_ud4nd_71").await?
                            }
                            "Discord" => default_get_job_details(&job, true, "body").await?,
                            "Palantir" => default_get_job_details(&job, true, ".content").await?,
                            "Anduril" => default_get_job_details(&job, true, "main").await?,
                            "Coinbase" => default_get_job_details(&job, false, ".Flex-sc-9cfb0d13-0.Listing__Container-sc-bcedfe82-0.fXHNQM.dBburU").await?,
                            _ => default_get_job_details(&job, true, "body").await?,
                        };

                                            // Print details
                                            // clear_console();
                                            job_details.print_job();
                                        }
                                        _ => break,
                                    }
                                }
                            }
                        }
                        "Add a Connection" => {
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
                        _ => {}
                    }
                }
            }
            "Network-Wide Scrape" => {
                //TODO: Scrape all jobs where connections > 0
                let companies_to_scrape: Vec<String> = data
                    .data
                    .iter()
                    .filter(|(_, c)| !c.connections.is_empty())
                    .map(|(k, _)| k.clone())
                    .collect();

                if companies_to_scrape.is_empty() {
                    println!("You must have at least 1 connection to a company to do this.");
                    sleep(Duration::from_secs(3));
                    continue;
                }

                let pb = ProgressBar::new(companies_to_scrape.len() as u64);
                pb.set_style(
    ProgressStyle::default_bar()
        .template("{spinner:.green} [{elapsed_precise}] {bar:60.cyan/blue} {pos}/{len} ({percent}%) {msg}")
        .unwrap()
        .progress_chars("=>-"),
);

                let mut new_jobs: Vec<FormattedJob> = vec![];

                // Enable steady ticks for animation
                pb.enable_steady_tick(Duration::from_millis(100));

                struct FormattedJob {
                    company: String,
                    display_name: String,
                    job: Job,
                }
                for company_key in companies_to_scrape {
                    clear_console();

                    // Set a message to show current activity
                    pb.set_message(format!("Scraping {}", company_key));

                    // Start timing the operation
                    let start = Instant::now();

                    // Perform the scraping
                    let jobs_payload = scrape_jobs(&mut data, &company_key).await?;

                    // Update progress and message
                    pb.inc(1);

                    if jobs_payload.are_new_jobs {
                        let new_jobs_count = jobs_payload.new_jobs.len();
                        pb.println(format!(
                            "✨ Found {} new jobs for {}!",
                            new_jobs_count, company_key
                        ));

                        for j in jobs_payload.new_jobs {
                            new_jobs.push(FormattedJob {
                                display_name: format!(
                                    "{} | {} | ({})",
                                    j.title, j.location, company_key
                                ),
                                job: j,
                                company: company_key.clone(),
                            });
                        }
                    }

                    // Optional: Show time taken for each company
                    let elapsed = start.elapsed();
                    pb.set_message(format!("Done in {:.2}s", elapsed.as_secs_f64()));
                }

                // Finish the progress bar
                pb.finish_with_message("Scraping completed!");

                if new_jobs.is_empty() {
                    clear_console();
                    println!("No new jobs have been detected :(");
                    sleep(Duration::from_secs(3));
                    continue;
                }

                let selected_job = &new_jobs[FuzzySelect::new()
                    .with_prompt("Select a job")
                    .items(
                        &new_jobs
                            .iter()
                            .map(|fj| &fj.display_name)
                            .collect::<Vec<&String>>(),
                    )
                    .interact()
                    .unwrap()]
                .job;

                sleep(Duration::from_secs(5));
                continue;
            }
            "My Connections" => {
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

                let table = Table::new(all_connections);

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
    match company_key {
        "Anduril" => Ok(default_scrape_jobs_handler(data, ANDURIL_SCRAPE_OPTIONS).await?),
        "Blizzard" => Ok(scrape_blizzard(data).await?),
        "Coinbase" => Ok(scrape_coinbase(data).await?),
        "Weedmaps" => Ok(default_scrape_jobs_handler(data, WEEDMAPS_SCRAPE_OPTIONS).await?),
        "1Password" => Ok(default_scrape_jobs_handler(data, ONEPASSWORD_SCRAPE_OPTIONS).await?),

        "Discord" => Ok(default_scrape_jobs_handler(data, DISCORD_SCRAPE_OPTIONS).await?),
        "Palantir" => Ok(default_scrape_jobs_handler(data, PALANTIR_DEFAULT_SCRAPE_OPTIONS).await?),
        "Reddit" => Ok(scrape_reddit(data).await?),

        "GitHub" => Ok(default_scrape_jobs_handler(data, GITHUB_SCRAPE_OPTIONS).await?),
        "GitLab" => Ok(default_scrape_jobs_handler(data, GITLAB_SCRAPE_OPTIONS).await?),
        "The Browser Company" => Ok(default_scrape_jobs_handler(
            data,
            THE_BROWSER_COMPANY_DEFAULT_SCRAPE_OPTIONS,
        )
        .await?),
        "Toast" => Ok(default_scrape_jobs_handler(data, TOAST_DEFAULT_SCRAPE_OPTIONS).await?),

        _ => panic!(),
    }
}
