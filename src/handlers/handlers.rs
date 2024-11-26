use std::{collections::HashSet, error::Error};

use clipboard::{ClipboardContext, ClipboardProvider};
use colored::*;
use dialoguer::{theme::ColorfulTheme, Confirm, Editor, FuzzySelect, Select};
use headless_chrome::{Browser, LaunchOptions};
use tabled::Table;

use crate::{
    models::{
        data::{Connection, Data},
        scraper::{Job, JobsPayload, ScrapedJob},
    },
    COMPANYKEYS,
};

use super::scrape_options::DefaultJobScraperOptions;

pub async fn default_scrape_jobs_handler(
    data: &mut Data,
    options: DefaultJobScraperOptions,
) -> Result<JobsPayload, Box<dyn Error>> {
    let launch_options = LaunchOptions {
        headless: options.headless,
        window_size: Some((1920, 1080)),
        enable_logging: true,

        ..LaunchOptions::default()
    };
    let browser = Browser::new(launch_options)?;

    let tab = browser.new_tab()?;

    tab.navigate_to(options.url)?;
    tab.wait_for_element("body")?;
    tab.wait_for_element(options.content_selector)?;

    let engineering_jobs = tab.evaluate(&options.get_jobs_js, false)?;

    let scraped_jobs: Vec<ScrapedJob> =
        serde_json::from_str(engineering_jobs.value.unwrap().as_str().unwrap()).unwrap();

    let jobs_payload =
        JobsPayload::from_scraped_jobs(scraped_jobs, &data.data[options.company_key]);

    data.data.get_mut(options.company_key).unwrap().jobs = jobs_payload.all_jobs.clone();

    data.save();

    Ok(jobs_payload)
}

pub fn prompt_user_for_company_selection() -> &'static str {
    let dialoguer_styles = ColorfulTheme::default();
    let mut company_options = COMPANYKEYS.to_vec();

    company_options.sort();

    company_options.push("Back");

    let selection = FuzzySelect::with_theme(&dialoguer_styles)
        .with_prompt("What do you choose?")
        .items(&company_options)
        .interact()
        .unwrap();

    return company_options[selection];
}

pub fn prompt_user_for_company_option(company: &'static str) -> &'static str {
    let dialoguer_styles = ColorfulTheme::default();
    let options = [
        "Scrape Jobs",
        "Add a Connection",
        "View/Edit Connections",
        "View Jobs",
        "Back",
    ];

    let selection = Select::with_theme(&dialoguer_styles)
        .with_prompt(&format!("Select an option for {}", company))
        .items(&options)
        .interact()
        .unwrap();

    return options[selection];
}

#[derive(Clone, Copy)]
pub enum JobOption {
    OpenJobInBrowser,
    ReachOut,
    GenerateJobDetails,
    Back,
}
pub fn prompt_user_for_job_option(job: &Job) -> (JobOption, &'static str) {
    let prompt = format!("Select an option for {}", job.title);

    let dialoguer_styles = ColorfulTheme::default();

    let job_options = [
        (JobOption::OpenJobInBrowser, "Open Job in Browser"),
        (JobOption::ReachOut, "Reach Out to a Connection"),
        (
            JobOption::GenerateJobDetails,
            "Generate Job Details with AI",
        ),
        (JobOption::Back, "Back"),
    ];
    let job_options_selection = job_options[Select::with_theme(&dialoguer_styles)
        .with_prompt(prompt)
        .items(&job_options.iter().map(|j| j.1).collect::<Vec<&str>>())
        .interact()
        .unwrap()];

    job_options_selection
}

pub fn handle_view_edit_connections() {}

pub fn handle_craft_a_message(job: &Job, connection: &Connection) {
    let dialoguer_styles = ColorfulTheme::default();
    let mut message = Editor::new().edit("Craft your message").unwrap().unwrap();

    message += &format!("\n\n{}", job.link);
    let mut clipboard: ClipboardContext = ClipboardProvider::new().unwrap();
    clipboard.set_contents(message.clone()).unwrap();

    println!("Your message has been copied to your clipboard.");

    if connection.linkedin.is_some() {
        let open_linked_in = Confirm::with_theme(&dialoguer_styles)
            .with_prompt("Open LinkedIn?")
            .interact()
            .unwrap();
        if open_linked_in {
            let _ = webbrowser::open(&connection.linkedin.as_ref().unwrap());
        }
    }
}

#[derive(Clone, Copy)]
pub enum ReachOutToAConnectionOption {
    CraftAMessage,
    OpenLinkedIn,
    Back,
}

pub fn prompt_user_for_connection_selection(connections: &Vec<Connection>) -> &Connection {
    let dialoguer_styles = ColorfulTheme::default();

    // TODO: If connections is empty, print message and continue loop

    let display_strings: Vec<String> = connections
        .iter()
        .map(|c| format!("{} {} ({})", c.first_name, c.last_name, c.role))
        .collect();

    let idx = Select::with_theme(&dialoguer_styles)
        .with_prompt("Select a connection")
        .items(&display_strings)
        .interact()
        .unwrap();

    let selected_connection = &connections[idx];

    let connection_table = Table::new(vec![selected_connection]);

    println!("{}", connection_table);

    selected_connection
}
pub fn prompt_user_for_connection_option(
    selected_connection: &Connection,
) -> ReachOutToAConnectionOption {
    let dialoguer_styles = ColorfulTheme::default();

    let mut connections_options = vec![
        (
            ReachOutToAConnectionOption::CraftAMessage,
            "Craft a Message",
        ),
        (ReachOutToAConnectionOption::Back, "Back"),
    ];

    if selected_connection.linkedin.is_some() {
        connections_options.insert(
            0,
            (ReachOutToAConnectionOption::OpenLinkedIn, "Open LinkedIn"),
        )
    }

    let selected_connection_option = connections_options[Select::with_theme(&dialoguer_styles)
        .with_prompt("Select an option")
        .items(
            &connections_options
                .iter()
                .map(|o| o.1)
                .collect::<Vec<&str>>(),
        )
        .interact()
        .unwrap()];

    selected_connection_option.0
}

pub fn handle_reach_out_to_a_connection(
    connections: &Vec<Connection>,
    selected_job: &Job,
) -> Result<bool, Box<dyn Error>> {
    let selected_connection = prompt_user_for_connection_selection(connections);

    let reach_out_to_a_connection_option = prompt_user_for_connection_option(selected_connection);

    match reach_out_to_a_connection_option {
        ReachOutToAConnectionOption::OpenLinkedIn => {
            let linkedin_url = selected_connection.linkedin.as_ref().unwrap();

            webbrowser::open(linkedin_url)?;

            Ok(false)
        }
        ReachOutToAConnectionOption::CraftAMessage => {
            handle_craft_a_message(selected_job, &selected_connection);

            Ok(false)
        }

        ReachOutToAConnectionOption::Back => Ok(true),
    }
}

pub fn prompt_user_for_job_selection(jobs: Vec<Job>, new_jobs: Option<Vec<Job>>) -> Option<Job> {
    struct FormattedJob<'a> {
        display_string: String,
        original_job: &'a Job,
    }
    let mut formatted_options = jobs
        .iter()
        .map(|j| {
            let mut display_string = format!("🧳 {} | 🌎 {}", j.title, j.location);

            if let Some(nj) = &new_jobs {
                let new_job = nj.iter().any(|nj| j.id == nj.id);
                if new_job {
                    display_string += " ❗".bright_green().bold().to_string().as_str();
                }
            }

            if !j.is_seen {
                display_string += " 👀".bright_green().bold().to_string().as_str();
            }

            FormattedJob {
                display_string,
                original_job: j,
            }
        })
        .collect::<Vec<FormattedJob>>();

    // INFO: Filter jobs down by locations if data set too large
    if jobs.len() > 99 {
        let locations = jobs
            .iter()
            .fold(HashSet::new(), |mut hash, job| {
                hash.insert(&job.location);

                return hash;
            })
            .into_iter()
            .collect::<Vec<&String>>();

        let dialoguer_styles = ColorfulTheme::default();
        let selection = Select::with_theme(&dialoguer_styles)
            .with_prompt("Select location")
            .items(&locations)
            .interact()
            .unwrap();

        let selected_location = locations[selection];

        formatted_options.retain(|j| &j.original_job.location == selected_location);
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
        return None;
    }

    let job = formatted_options[selected_job].original_job;

    Some(job.clone())
}
