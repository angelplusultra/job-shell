use core::panic;
use dialoguer::theme::ColorfulTheme;
use dialoguer::{Confirm, FuzzySelect, Select};
use dotenv::dotenv;
use headless_chrome::{Browser, LaunchOptions};
use models::gemini::{GeminiClient, GeminiJob};
use models::scraper::{Job, JobsPayload};
use models::snapshots::{self, Snapshots};
use scrapers::anduril::scraper::scrape_anduril;
use scrapers::onepassword::scraper::scrape_1password;
use scrapers::square::scraper::scrape_square;
use scrapers::tarro::scraper::scrape_tarro;
use scrapers::weedmaps::job_details::get_weedmaps_jod_details;
use scrapers::weedmaps::scraper::scrape_weedmaps;
use std::collections::HashSet;
use std::env;
use std::error::Error;
use std::io::Write;
use std::thread::sleep;
use std::time::Duration;
use webbrowser;

// mod links
mod utils {
    pub mod snapshots;
    pub mod stringify_js;
}
mod models {
    pub mod custom_error;
    pub mod gemini;
    pub mod scraper;
    pub mod snapshots;
}
mod scrapers {
    pub mod weedmaps {
        pub mod job_details;
        pub mod scraper;
    }
    pub mod onepassword {
        pub mod scraper;
    }

    pub mod square {
        pub mod scraper;
    }
    pub mod tarro {
        pub mod scraper;
    }
    pub mod anduril {
        pub mod scraper;
    }
}

mod handlers;
/*
*
*
*/


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

    let body = tab.wait_for_element("body")?;
    let content = tab.wait_for_element(content_selector)?;

    let html = content.get_content()?;

    let gemini_job = GeminiJob::from_job_html(html).await?;

    return Ok(gemini_job);
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    clear_console();
    dotenv().ok();

    let welcome = figlet_rs::FIGfont::from_file("src/fonts/slant.flf").unwrap();

    let logo = welcome.convert("Skibidi Skraper").unwrap();

    println!("{logo}");
    sleep(Duration::from_secs(3));
    let mut app_active = true;

    'main: while app_active {
        clear_console();

        let mut snapshots = Snapshots::get_snapshots();

        let mut companies = [
            "Anduril",
            "Weedmaps",
            "1Password",
            "Tarro",
            "Toast",
            "GitHub",
            "GitLab",
            "Palantir",
            "Meta",
            "Wynn",
            "Cloudflare",
        ]
        .to_vec();

        companies.sort();

        companies.push("Exit");

        // Get company Selection
        let selection = FuzzySelect::new()
            .with_prompt("What do you choose?")
            .items(&companies)
            .interact()
            .unwrap();

        let company = companies[selection];

        if company == "Exit" {
            break;
        }

        let options = ["Scrape Jobs", "Add a Connection"];
        let selection = Select::with_theme(&ColorfulTheme::default())
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
                    "Anduril" => scrape_anduril(&mut snapshots).await?,
                    "Cloudflare" => todo!(),
                    "Indeed" => todo!(),
                    "1Password" => scrape_1password(&mut snapshots).await?,
                    "Square" => scrape_square(&mut snapshots).await?,
                    "Tarro" => scrape_tarro(&mut snapshots).await?,
                    "Weedmaps" => scrape_weedmaps(&mut snapshots).await?,
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

                    let selection = Select::new()
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
                    let selected_job = FuzzySelect::with_theme(&ColorfulTheme::default())
                        .with_prompt("Select a job")
                        .items(&display_options)
                        .interact()
                        .unwrap();

                    let job = formatted_options[selected_job].original_job;

                    // INFO: Get Job Details from AI
                    let job_details = match company {
                        "Weedmaps" => get_weedmaps_jod_details(&job).await?,
                        "1Password" => default_get_job_details(&job, true, "body").await?,
                        "Tarro" => {
                            default_get_job_details(&job, true, "._content_ud4nd_71").await?
                        }
                        "Anduril" => default_get_job_details(&job, true, "main").await?,
                        _ => panic!(),
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
                    let selection = Select::with_theme(&ColorfulTheme::default())
                        .with_prompt("Select an option")
                        .items(&options)
                        .interact()
                        .unwrap();

                    match options[selection] {
                        // The user wants to apply
                        "Apply" => {
                            clear_console();
                            webbrowser::open(&job.link)?;

                            let apply = Confirm::new()
                                .with_prompt("Did you apply?")
                                .interact()
                                .unwrap();

                            if apply {
                                //TODO: Mark job as applied
                            }

                            clear_console();
                            // Ask the user if they'd like details on another job;
                            let get_another_job_details = Confirm::new()
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
