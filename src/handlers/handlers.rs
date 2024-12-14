use std::{
    collections::HashSet,
    error::Error,
    fmt::Display,
    thread::sleep,
    time::{Duration, Instant},
};

use clipboard::{ClipboardContext, ClipboardProvider};
use colored::*;
use dialoguer::{theme::ColorfulTheme, Confirm, Editor, FuzzySelect, Select};
use headless_chrome::{Browser, LaunchOptions};
use indicatif::{ProgressBar, ProgressStyle};
use strum::IntoEnumIterator;
use tabled::Table;

use crate::{
    models::{
        data::{Company, Connection, Data},
        scraper::{Job, JobsPayload, ScrapedJob},
    },
    reports::{create_report, ReportMode},
    scrape_jobs,
    utils::{clear_console, stall_and_present_countdown},
    COMPANYKEYS,
};

use super::scrape_options::DefaultJobScraperOptions;
use strum_macros::{Display, EnumIter};

// INFO: Display string fn for all enums that derive EnumIter
trait EnumVariantsDisplayStrings: IntoEnumIterator + Display {
    fn display_strings() -> Vec<String> {
        Self::iter().map(|v| v.to_string()).collect()
    }
}

impl<T: IntoEnumIterator + Display> EnumVariantsDisplayStrings for T {}

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

// INFO: Company Options Prompt
#[derive(Display, EnumIter)]
pub enum CompanyOption {
    #[strum(to_string = "Scrape and Update Jobs")]
    ScrapeAndUpdateJobs,
    #[strum(to_string = "View Jobs")]
    ViewJobs,
    #[strum(to_string = "Add a Connection")]
    AddAConnection,
    #[strum(to_string = "View or Edit Connections")]
    ViewOrEditConnections,
    #[strum(to_string = "Follow Company [ ]")]
    FollowCompany,
    #[strum(to_string = "Back")]
    Back,
}

pub fn prompt_user_for_company_option(
    company_name: &'static str,
    is_following: bool,
) -> CompanyOption {
    let dialoguer_styles = ColorfulTheme::default();

    let mut options = CompanyOption::display_strings();

    if is_following {
        let idx = CompanyOption::iter()
            .position(|o| matches!(o, CompanyOption::FollowCompany))
            .unwrap();
        options[idx] = format!("Follow Company [x]")
    }

    let selection = Select::with_theme(&dialoguer_styles)
        .with_prompt(&format!("Select an option for {}", company_name))
        .items(&options)
        .interact()
        .unwrap();

    CompanyOption::iter().nth(selection).unwrap()
}

// INFO: Job Option Prompt
#[derive(Display, EnumIter)]
pub enum JobOption {
    #[strum(to_string = "Open Job in Browser")]
    OpenJobInBrowser,
    #[strum(to_string = "Reach Out to a Connection")]
    ReachOut,
    #[strum(to_string = "Bookmark Job [ ]")]
    Bookmark,
    #[strum(to_string = "Generate Job Details with AI (Experimental)")]
    GenerateJobDetails,
    #[strum(to_string = "Back")]
    Back,
}

pub fn prompt_user_for_job_option(job: &Job) -> JobOption {
    let prompt = format!("Select an option for {}", job.title);

    let dialoguer_styles = ColorfulTheme::default();

    let mut options = JobOption::display_strings();

    if job.is_bookmarked {
        let idx = JobOption::iter()
            .position(|o| matches!(o, JobOption::Bookmark))
            .unwrap();
        options[idx] = format!("Bookmark Job [x]")
    }

    let job_options_selection = Select::with_theme(&dialoguer_styles)
        .with_prompt(prompt)
        .items(&options)
        .interact()
        .unwrap();

    JobOption::iter().nth(job_options_selection).unwrap()
}

// pub fn handle_view_edit_connections() {}

// INFO: Open Job in Browser
pub fn handle_open_job_in_browser(job: &Job, data: &mut Data) -> Result<(), Box<dyn Error>> {
    webbrowser::open(&job.link)?;

    let dialoguer_styles = ColorfulTheme::default();
    let apply = Confirm::with_theme(&dialoguer_styles)
        .with_prompt("Did you apply?")
        .interact()
        .unwrap();

    if apply {
        data.mark_job_applied(&job.id);
    }

    Ok(())
}

// INFO: Craft a Message
pub fn handle_craft_a_message(job: &Job, connection: &Connection) {
    let dialoguer_styles = ColorfulTheme::default();
    let mut message = Editor::new().edit("Craft your message").unwrap().unwrap();

    message += &format!("\n\n{}", job.link);
    let mut clipboard: ClipboardContext = ClipboardProvider::new().unwrap();
    clipboard.set_contents(message.clone()).unwrap();

    clear_console();
    stall_and_present_countdown(
        3,
        Some("Your message has been copied to your clipboard along with the link to the job!"),
    );

    clear_console();
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

// INFO: Prompt for Connection Option
#[derive(Display, EnumIter)]
pub enum ConnectionOption {
    #[strum(to_string = "Craft a Message")]
    CraftAMessage,
    #[strum(to_string = "Open LinkedIn")]
    OpenLinkedIn,
    #[strum(to_string = "Back")]
    Back,
}
pub fn prompt_user_for_connection_option(selected_connection: &Connection) -> ConnectionOption {
    let dialoguer_styles = ColorfulTheme::default();

    let mut options = ConnectionOption::display_strings();

    if selected_connection.linkedin.is_none() {
        options.retain(|opt| opt != "Open LinkedIn");
    }

    let idx = Select::with_theme(&dialoguer_styles)
        .with_prompt("Select an option")
        .items(&options)
        .interact()
        .unwrap();

    ConnectionOption::iter().nth(idx).unwrap()
}

pub fn handle_reach_out_to_a_connection(
    connections: &Vec<Connection>,
    selected_job: &Job,
) -> Result<(), Box<dyn Error>> {
    clear_console();
    if connections.is_empty() {
        stall_and_present_countdown(3, Some("You currently have no connections at this company"));
    } else {
        let selected_connection = prompt_user_for_connection_selection(connections);

        let reach_out_to_a_connection_option =
            prompt_user_for_connection_option(selected_connection);

        match reach_out_to_a_connection_option {
            ConnectionOption::OpenLinkedIn => {
                let linkedin_url = selected_connection.linkedin.as_ref().unwrap();

                webbrowser::open(linkedin_url)?;
            }
            ConnectionOption::CraftAMessage => {
                handle_craft_a_message(selected_job, &selected_connection);
            }

            ConnectionOption::Back => {}
        }
    }

    Ok(())
}

pub fn handle_job_selection(
    jobs: Vec<Job>,
    new_jobs: Option<Vec<Job>>,
    company_name: &'static str,
) -> Option<Job> {
    #[derive(Clone)]
    struct FormattedJob<'a> {
        display_string: String,
        original_job: &'a Job,
    }

    fn prompt_user_for_job_selection(
        formatted_options: Vec<FormattedJob>,
        company_name: &str,
    ) -> Option<Job> {
        let mut display_options = formatted_options
            .iter()
            .map(|j| j.display_string.as_str())
            .collect::<Vec<&str>>();

        let prompt = format!(
            "Select a job @ {} ({}, 👀: Unseen, ❗: New Listing)",
            company_name,
            display_options.len()
        );
        // Pushing Exit option
        display_options.push("Back");

        // Prompt user for job selection
        let selected_job = FuzzySelect::new()
            .with_prompt(&prompt)
            .items(&display_options)
            .interact()
            .unwrap();

        if display_options[selected_job] == "Back" {
            return None;
        }

        let job = formatted_options[selected_job].original_job;

        Some(job.clone())
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
    if jobs.len() > 199 {
        let mut locations = jobs
            .iter()
            .fold(HashSet::new(), |mut hash, job| {
                hash.insert(&job.location);

                return hash;
            })
            .into_iter()
            .cloned()
            .collect::<Vec<String>>();

        locations.push("Back".to_string());

        let dialoguer_styles = ColorfulTheme::default();

        loop {
            let selection = FuzzySelect::with_theme(&dialoguer_styles)
                .with_prompt("Select location")
                .items(&locations)
                .interact()
                .unwrap();

            let selected_location = &locations[selection];

            if selected_location == "Back" {
                return None;
            }

            formatted_options.retain(|j| j.original_job.location == *selected_location);

            if let Some(j) = prompt_user_for_job_selection(formatted_options.clone(), company_name)
            {
                return Some(j);
            } else {
                continue;
            }
        }
    }
    if let Some(j) = prompt_user_for_job_selection(formatted_options, company_name) {
        return Some(j);
    } else {
        return None;
    }
}

pub struct FormattedJob {
    pub company: String,
    pub display_name: String,
    pub job: Job,
}
pub async fn handle_scan_new_jobs_across_network_and_followed_companies(
    data: &mut Data,
) -> Result<Vec<FormattedJob>, Box<dyn Error>> {
    clear_console();
    let companies_to_scrape: Vec<String> = data
        .data
        .iter()
        // Filter out companies where connections.len() > 0 or company.is_following equals true
        .filter(|(_, c)| {
            if !c.connections.is_empty() || c.is_following {
                return true;
            } else {
                return false;
            }
        })
        .map(|(k, _)| k.clone())
        .collect();

    if companies_to_scrape.is_empty() {
        return Err("Oops! Looks like you’re not connected with any companies yet or following any. Start building your network by adding connections or following companies you’re interested in!".into());
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

    for company_key in companies_to_scrape {
        clear_console();

        // Set a message to show current activity
        pb.set_message(format!("Scraping {}", company_key));

        // Start timing the operation
        let start = Instant::now();

        // Perform the scraping
        let jobs_payload = match scrape_jobs(data, &company_key).await {
            Ok(jb) => jb,
            Err(e) => {
                eprintln!("{}", format!("Error: {}", e).red());
                continue;
            }
        };

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
                    display_name: format!("{} | {} | ({})", j.title, j.location, company_key),
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
        return Err("No new jobs have been detcted :(".into());
    }

    create_report(&new_jobs, ReportMode::HTML)?;

    Ok(new_jobs)
}

#[derive(Display, EnumIter)]
pub enum MainMenuOption {
    #[strum(to_string = "Select a Company")]
    SelectACompany,
    #[strum(to_string = "Scan for New Jobs Across Network and Followed Companies")]
    ScanForNewJobsAcrossNetworkAndFollowedCompanies,
    #[strum(to_string = "View Bookmarked Jobs")]
    ViewBookmarkedJobs,
    #[strum(to_string = "My Connections")]
    MyConnections,
    #[strum(to_string = "View New Jobs Reports")]
    ViewNewJobsReports,
    #[strum(to_string = "Exit")]
    Exit,
}
pub fn prompt_user_for_main_menu_selection() -> MainMenuOption {
    let dialoguer_styles = ColorfulTheme::default();
    let options = MainMenuOption::display_strings();

    let idx = Select::with_theme(&dialoguer_styles)
        .with_prompt("Select an option")
        .items(&options)
        .interact()
        .unwrap();

    return MainMenuOption::iter().nth(idx).unwrap();
}

pub async fn handle_manage_connection(
    connection: &Connection,
    data: &mut Data,
    company_name: &str,
) -> Result<(), Box<dyn Error>> {
    let dialoguer_styles = ColorfulTheme::default();
    #[derive(EnumIter, Display)]
    enum ManageConnectionOption {
        #[strum(to_string = "Open LinkedIn")]
        OpenLinkedIn,
        #[strum(to_string = "Delete")]
        Delete,
        #[strum(to_string = "Back")]
        Back,
    }

    loop {
        let idx = FuzzySelect::with_theme(&dialoguer_styles)
            .with_prompt("Select an option")
            .items(&ManageConnectionOption::display_strings())
            .interact()
            .unwrap();

        let selected_option = ManageConnectionOption::iter().nth(idx).unwrap();

        match selected_option {
            ManageConnectionOption::OpenLinkedIn => {
                if let Some(linkedin_url) = &connection.linkedin {
                    webbrowser::open(linkedin_url)?
                } else {
                    println!("Connection does not have a LinkedIn URL set");
                }
            }
            ManageConnectionOption::Back => {
                break;
            }
            ManageConnectionOption::Delete => {
                let confirm = Confirm::with_theme(&dialoguer_styles)
                    .with_prompt(format!(
                        "Are you sure you want to delete {} {}?",
                        connection.first_name, connection.last_name
                    ))
                    .interact()?;

                if confirm {
                    // Filter out the connection from all companies
                    if let Some(company) = data.data.get_mut(company_name) {
                        company.connections.retain(|c| {
                            c.first_name != connection.first_name
                                || c.last_name != connection.last_name
                        });
                        data.save();
                        println!("Connection deleted successfully!");
                    } else {
                        println!("Something went wrong")
                    }

                    break;
                }
            }
        }
    }

    Ok(())
}
