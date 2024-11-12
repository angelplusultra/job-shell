use core::panic;
use dialoguer::theme::ColorfulTheme;
use dialoguer::{Confirm, FuzzySelect, Select};
use dotenv::dotenv;
use handlers::handlers::default_scrape_jobs_handler;
use handlers::scrape_options::{
    ANDURIL_SCRAPE_OPTIONS, DISCORD_SCRAPE_OPTIONS, ONEPASSWORD_SCRAPE_OPTIONS,
    WEEDMAPS_SCRAPE_OPTIONS,
};
use headless_chrome::{Browser, LaunchOptions};
use models::data::Data;
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
const COMPANYKEYS: [&str; 5] = ["Anduril", "1Password", "Weedmaps", "Discord", "Reddit"];
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
    let mut app_active = true;

    // INFO: Main App loop
    loop {
        clear_console();

        let mut data = Data::get_data();

        let mut companies = COMPANYKEYS.to_vec();

        companies.sort();

        companies.push("Exit");

        // Get company Selection
        let selection = FuzzySelect::with_theme(&dialoguer_styles)
            .with_prompt("What do you choose?")
            .items(&companies)
            .interact()
            .unwrap();

        let company = companies[selection];

        if company == "Exit" {
            break;
        }

        let options = ["Scrape Jobs", "Add a Connection"];
        let selection = Select::with_theme(&dialoguer_styles)
            .with_prompt("Select an option")
            .items(&options)
            .interact()
            .unwrap();

        match options[selection] {
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
                        default_scrape_jobs_handler(&mut data, ONEPASSWORD_SCRAPE_OPTIONS).await?
                    }

                    "Discord" => {
                        default_scrape_jobs_handler(&mut data, DISCORD_SCRAPE_OPTIONS).await?
                    }
                    "Reddit" => scrape_reddit(&mut data).await?,

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

                        let mut display_string = format!("{} {}", j.title, j.location);
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

                let mut selected_job_loop = true;
                while selected_job_loop {
                    let display_options = formatted_options
                        .iter()
                        .map(|j| &j.display_string)
                        .collect::<Vec<&String>>();
                    let selected_job = FuzzySelect::with_theme(&dialoguer_styles)
                        .with_prompt("Select a job")
                        .items(&display_options)
                        .interact()
                        .unwrap();

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

                    // Confirm to apply
                    // let apply = Confirm::new()
                    //     .with_prompt("Want to apply?")
                    //     .interact()
                    //     .unwrap();

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
                            let get_another_job_details = Confirm::with_theme(&dialoguer_styles)
                                .with_prompt("Would you like to get the details of another job?")
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
                            todo!();
                        }
                        _ => {}
                    }
                }
            }
            "Add a Connection" => {
                // TODO: Add a connections handler
            }
            _ => {}
        }
    }

    Ok(())
}
