use clipboard::{ClipboardContext, ClipboardProvider};
use colored::*;
use core::panic;
use dialoguer::theme::ColorfulTheme;
use dialoguer::{Confirm, Editor, FuzzySelect, Input, Select};
use dotenv::dotenv;
use handlers::handlers::default_scrape_jobs_handler;
use handlers::scrape_options::{
    ANDURIL_SCRAPE_OPTIONS, DISCORD_SCRAPE_OPTIONS, GITHUB_SCRAPE_OPTIONS,
    ONEPASSWORD_SCRAPE_OPTIONS, WEEDMAPS_SCRAPE_OPTIONS,
};
use headless_chrome::{Browser, LaunchOptions};
use models::data::{Company, Connection, Data};
use models::gemini::GeminiJob;
use models::scraper::{Job, JobsPayload};
use scrapers::reddit::scraper::scrape_reddit;
use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::io::Write;
use std::thread::sleep;
use std::time::Duration;
use tabled::Table;
use webbrowser;

// TODO: Keys should prob be lowercase, make a tuple where 0 is key and 1 is display name
const COMPANYKEYS: [&str; 6] = [
    "Anduril",
    "1Password",
    "Weedmaps",
    "Discord",
    "Reddit",
    "GitHub",
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

        let mut main_options = COMPANYKEYS.to_vec();

        main_options.sort();

        // main_options.push("Scrape My Connection Jobs");
        // main_options.push("View All Connections");
        main_options.push("Exit");

        // Get company Selection
        let selection = FuzzySelect::with_theme(&dialoguer_styles)
            .with_prompt("What do you choose?")
            .items(&main_options)
            .interact()
            .unwrap();

        let main_selection = main_options[selection];

        if main_selection == "Exit" {
            break;
        }

        if main_selection == "View All Connections" {
            let all_connections: Vec<&Connection> = data
                .data
                .iter()
                .map(|(_, c)| &c.connections)
                .flatten()
                .collect();

            let table = Table::new(all_connections);

            println!("{}", table);

            Input::<String>::new()
                .with_prompt("Press enter to continue")
                .interact()
                .unwrap();

            continue;
        }

        if main_selection == "Scrape My Connection Jobs" {
            // TODO: ugh huge todo
            let companies_to_scrape: Vec<String> = data
                .data
                .iter()
                .filter(|(_, c)| !c.connections.is_empty())
                .map(|(k, _)| k.clone())
                .collect();

            println!("{:?}", companies_to_scrape);

            for company_key in companies_to_scrape {
                println!("Scraping {}", company_key);
                let jobs_payload = scrape_jobs(&mut data, &company_key).await?;

                if jobs_payload.are_new_jobs {
                    let string = format!("{} has new jobs!", company_key);
                    println!("{string}");
                }

                println!("Finished scraping {}", company_key);
            }

            sleep(Duration::from_secs(5));
            continue;
        }

        let company = main_selection;

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
                    let selected_connection_option =
                        connections_options[Select::with_theme(&dialoguer_styles)
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

                    // INFO: Format jobs for presentation
                    let mut formatted_options = all_jobs
                        .iter()
                        .map(|j| {
                            let new_job = new_jobs.iter().find(|nj| j.title == nj.title);

                            let mut display_string = format!("🧳 {} | 🌎 {}", j.title, j.location);
                            if new_job.is_some() {
                                display_string += " NEW!!!";
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

                        formatted_options.retain(|j| &j.original_job.location == selected_location);
                    }

                    // INFO: Job Selection Loop
                    loop {
                        let mut display_options = formatted_options
                            .iter()
                            .map(|j| j.display_string.as_str())
                            .collect::<Vec<&str>>();

                        // Pushing Exit option
                        display_options.push("Exit");

                        let selected_job = FuzzySelect::with_theme(&dialoguer_styles)
                            .with_prompt("Select a job")
                            .items(&display_options)
                            .interact()
                            .unwrap();

                        if display_options[selected_job] == "Exit" {
                            break;
                        }

                        let job = formatted_options[selected_job].original_job;

                        // INFO: Get Job Details from AI
                        let job_details = match company {
                            // "Weedmaps" => get_weedmaps_jod_details(&job).await?,
                            "1Password" => default_get_job_details(&job, true, "body").await?,
                            "Tarro" => {
                                default_get_job_details(&job, true, "._content_ud4nd_71").await?
                            }
                            "Discord" => default_get_job_details(&job, true, "body").await?,
                            "Anduril" => default_get_job_details(&job, true, "main").await?,
                            _ => default_get_job_details(&job, true, "body").await?,
                        };

                        // Print details
                        clear_console();
                        job_details.print_job();

                        //TODO: shove below in loop above
                        let options = ["Apply", "Reach out to a connection", "Back"];
                        let selection = Select::with_theme(&dialoguer_styles)
                            .with_prompt("Select an option")
                            .items(&options)
                            .interact()
                            .unwrap();

                        match options[selection] {
                            // The user wants to apply
                            "Apply" => {
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
                                            .find(|j| j.title == job.title)
                                            .unwrap();
                                        selected_job.applied = true;

                                        data.save();
                                    }
                                }

                                clear_console();
                                // Ask the user if they'd like details on another job;
                                let get_another_job_details =
                                    Confirm::with_theme(&dialoguer_styles)
                                        .with_prompt(
                                            "Would you like to get the details of another job?",
                                        )
                                        .interact()
                                        .unwrap();

                                match get_another_job_details {
                                    // if they do, repeat the loop
                                    true => {
                                        continue;
                                    }
                                    // break the inner loop, starting the app over
                                    _ => {
                                        // selected_job_loop = false;
                                        break;
                                    }
                                }
                            }
                            // INFO: Reach out to a connection handler
                            "Reach out to a connection" => {
                                let connections = &data.data[company].connections;

                                // TODO: If connections is empty, print message and continue loop

                                let display_strings: Vec<String> = connections
                                    .iter()
                                    .map(|c| {
                                        format!("{} {} ({})", c.first_name, c.last_name, c.role)
                                    })
                                    .collect();

                                let idx = Select::with_theme(&dialoguer_styles)
                                    .with_prompt("Select a connection")
                                    .items(&display_strings)
                                    .interact()
                                    .unwrap();

                                let selected_connection = &connections[idx];

                                let connection_table = Table::new(vec![selected_connection]);

                                println!("{}", connection_table);

                                let mut connections_options = vec!["Craft a Message", "Back"];

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
                                        let linkedin_url =
                                            selected_connection.linkedin.as_ref().unwrap();

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
                                        clipboard.set_contents(message.clone()).unwrap();

                                        println!("Your message has been copied to your clipboard.");

                                        if selected_connection.linkedin.is_some() {
                                            let open_linkedIn =
                                                Confirm::with_theme(&dialoguer_styles)
                                                    .with_prompt("Open LinkedIn?")
                                                    .interact()
                                                    .unwrap();
                                            if open_linkedIn {
                                                webbrowser::open(
                                                    &selected_connection.linkedin.as_ref().unwrap(),
                                                );
                                            }
                                        }
                                    }

                                    "Back" => break,
                                    _ => panic!(),
                                }
                            }
                            _ => {}
                        }
                    }
                }
                "Add a Connection" => {
                    clear_console();
                    println!("Create a new connection at {}", company);

                    let new_connection = Connection::create_with_form(&dialoguer_styles);

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

                    // data.save();
                }
                _ => {}
            }
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
        "Weedmaps" => Ok(default_scrape_jobs_handler(data, WEEDMAPS_SCRAPE_OPTIONS).await?),
        "1Password" => Ok(default_scrape_jobs_handler(data, ONEPASSWORD_SCRAPE_OPTIONS).await?),

        "Discord" => Ok(default_scrape_jobs_handler(data, DISCORD_SCRAPE_OPTIONS).await?),
        "Reddit" => Ok(scrape_reddit(data).await?),

        "GitHub" => Ok(default_scrape_jobs_handler(data, GITHUB_SCRAPE_OPTIONS).await?),

        _ => panic!(),
    }
}
