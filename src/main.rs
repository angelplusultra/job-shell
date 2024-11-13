use colored::*;
use tabled::Table;
use core::panic;
use dialoguer::theme::ColorfulTheme;
use dialoguer::{Confirm, FuzzySelect, Input, Select};
use dotenv::dotenv;
use handlers::handlers::default_scrape_jobs_handler;
use handlers::scrape_options::{
    ANDURIL_SCRAPE_OPTIONS, DISCORD_SCRAPE_OPTIONS, GITHUB_SCRAPE_OPTIONS,
    ONEPASSWORD_SCRAPE_OPTIONS, WEEDMAPS_SCRAPE_OPTIONS,
};
use headless_chrome::{Browser, LaunchOptions};
use models::data::{Connection, Data};
use models::gemini::GeminiJob;
use models::scraper::{Job, JobsPayload};
use scrapers::reddit::scraper::scrape_reddit;
use std::collections::HashSet;
use std::error::Error;
use std::io::Write;
use std::thread::sleep;
use std::time::Duration;
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

    let gemini_job = GeminiJob::from_job_html(html).await?;

    return Ok(gemini_job);
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

        main_options.push("Scrape My Connection Jobs");
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

        if main_selection == "Scrape My Connection Jobs" {
            todo!();
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
            // TODO: Need to put below in this loop

            let selection = Select::with_theme(&dialoguer_styles)
                .with_prompt("Select an option")
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
                "Scrape Jobs" => {
                    //TODO: ScrapeJobs Handler
                    let JobsPayload {
                        all_jobs,
                        new_jobs,
                        are_new_jobs,
                    } = match company {
                        "Anduril" => {
                            default_scrape_jobs_handler(&mut data, ANDURIL_SCRAPE_OPTIONS).await?
                        }
                        "Weedmaps" => {
                            default_scrape_jobs_handler(&mut data, WEEDMAPS_SCRAPE_OPTIONS).await?
                        }
                        "1Password" => {
                            default_scrape_jobs_handler(&mut data, ONEPASSWORD_SCRAPE_OPTIONS)
                                .await?
                        }

                        "Discord" => {
                            default_scrape_jobs_handler(&mut data, DISCORD_SCRAPE_OPTIONS).await?
                        }
                        "Reddit" => scrape_reddit(&mut data).await?,

                        "GitHub" => {
                            default_scrape_jobs_handler(&mut data, GITHUB_SCRAPE_OPTIONS).await?
                        }

                        _ => panic!(),
                    };

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
                            // TODO: Reach out to connection
                            "Reach out to a connection" => {
                                
                            }
                            _ => {}
                        }
                    }
                }
                "Add a Connection" => {
                    // TODO: Add a connections handler
                    let first_name: String = Input::with_theme(&dialoguer_styles)
                        .with_prompt("Enter their first name")
                        .interact_text()
                        .unwrap();

                    let last_name: String = Input::with_theme(&dialoguer_styles)
                        .with_prompt("Enter their last name")
                        .interact_text()
                        .unwrap();

                    let current_employee = Confirm::with_theme(&dialoguer_styles)
                        .with_prompt("Are they currently employed at this company?")
                        .interact()
                        .unwrap();

                    let role: String = Input::with_theme(&dialoguer_styles)
                        .with_prompt("Enter their role at this company (e.g Software Engineer)")
                        .interact_text()
                        .unwrap();

                    let email: Option<String> = Input::with_theme(&dialoguer_styles)
                        .with_prompt("Enter their email (Press Enter to skip)")
                        .allow_empty(true)
                        .interact_text()
                        .ok()
                        .filter(|s: &String| !s.is_empty());

                    let linkedin: Option<String> = Input::with_theme(&dialoguer_styles)
                        .with_prompt("Enter their LinkedIn profile (Press Enter to skip)")
                        .allow_empty(true)
                        .interact_text()
                        .ok()
                        .filter(|s: &String| !s.is_empty());

                    let new_connection = Connection {
                        first_name,
                        last_name,
                        role,
                        current_employee,
                        email,
                        linkedin,
                    };

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

                    data.save();

                }
                _ => {}
            }
        }
    }

    Ok(())
}
