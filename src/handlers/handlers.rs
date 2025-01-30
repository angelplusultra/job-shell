use std::{
    collections::HashSet,
    error::Error,
    fmt::Display,
    thread::sleep,
    time::{Duration, Instant},
};

use clipboard::{ClipboardContext, ClipboardProvider};
use colored::*;
use dialoguer::{theme::ColorfulTheme, Confirm, Editor, FuzzySelect, Input, Select};
use headless_chrome::{Browser, LaunchOptions};
use indicatif::{ProgressBar, ProgressStyle};
use strum::IntoEnumIterator;
use tabled::Table;

use crate::{
    company_options::{CompanyOption, ScrapeJobs},
    error::AppResult,
    models::{
        ai::{AiModel, OpenAIClient},
        data::{Connection, Data},
        scraper::{Job, JobsPayload, ScrapedJob},
    },
    reports::{create_report, ReportMode},
    utils::{clear_console, stall_and_present_countdown},
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

// pub fn prompt_user_for_company_selection() -> &'static str {
//     let dialoguer_styles = ColorfulTheme::default();
//     let mut company_options = COMPANYKEYS.to_vec();
//
//     company_options.sort();
//
//     company_options.push("Back");
//
//     let selection = FuzzySelect::with_theme(&dialoguer_styles)
//         .with_prompt("What do you choose?")
//         .items(&company_options)
//         .interact()
//         .unwrap();
//
//     return company_options[selection];
// }

pub fn prompt_user_for_company_selection_v2() -> Option<CompanyOption> {
    let dialoguer_styles = ColorfulTheme::default();

    // Create a sorted list of CompanyOption instances
    let mut company_options: Vec<CompanyOption> = CompanyOption::iter().collect();
    company_options.sort_by_key(|opt| opt.to_string());

    // Add "Back" option
    let mut display_options = company_options
        .iter()
        .map(|opt| opt.to_string())
        .collect::<Vec<String>>();

    display_options.push("Back".to_string());

    let selection = FuzzySelect::with_theme(&dialoguer_styles)
        .with_prompt("Select a company")
        .items(&display_options)
        .interact()
        .unwrap();

    if selection == display_options.len() - 1 {
        return None;
    }

    // Return the selected company from the sorted list
    Some(company_options[selection].clone())
}
// INFO: Company Options Prompt
#[derive(Display, EnumIter)]
pub enum SelectedCompanyOption {
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
    company_name: &str,
    is_following: bool,
) -> SelectedCompanyOption {
    let dialoguer_styles = ColorfulTheme::default();

    let mut options = SelectedCompanyOption::display_strings();

    if is_following {
        let idx = SelectedCompanyOption::iter()
            .position(|o| matches!(o, SelectedCompanyOption::FollowCompany))
            .unwrap();
        options[idx] = format!("Follow Company [x]")
    }

    let selection = Select::with_theme(&dialoguer_styles)
        .with_prompt(&format!("Select an option for {}", company_name))
        .items(&options)
        .interact()
        .unwrap();

    SelectedCompanyOption::iter().nth(selection).unwrap()
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
    // #[strum(to_string = "Generate Job Details with AI (Experimental)")]
    // GenerateJobDetails,
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
pub fn handle_open_job_in_browser(job: &Job, data: &mut Data) -> AppResult<()> {
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
) -> AppResult<()> {
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
    company_name: &str,
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

            let filtered_jobs_by_location: Vec<FormattedJob> = formatted_options
                .iter()
                .filter(|j| j.original_job.location == *selected_location)
                .cloned()
                .collect();

            if let Some(j) = prompt_user_for_job_selection(filtered_jobs_by_location, company_name)
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
) -> AppResult<Vec<FormattedJob>> {
    clear_console();
    let companies_to_scrape: Vec<CompanyOption> = CompanyOption::iter()
        .filter(|c| {
            let company_key = c.to_string();
            let company = data.companies.get(&company_key).unwrap();
            if !company.connections.is_empty() || company.is_following {
                return true;
            } else {
                return false;
            }
        })
        .collect();

    if companies_to_scrape.is_empty() {
        return Err("Looks like you’re not connected with any companies yet or following any. Start building your network by adding connections or following companies you’re interested in!".into());
    }

    let pb = ProgressBar::new(companies_to_scrape.len() as u64);
    pb.set_style(
    ProgressStyle::default_bar()
        .template("{spinner:.green} [{elapsed_precise}] {bar:60.cyan/blue} {pos}/{len} ({percent}%) {msg}")
        .unwrap()
        .progress_chars("=>-"),
);

    let mut new_jobs_based_on_smart_criteria: Vec<FormattedJob> = vec![];
    let mut all_new_jobs: Vec<FormattedJob> = vec![];

    // Enable steady ticks for animation
    pb.enable_steady_tick(Duration::from_millis(100));

    for company_option in companies_to_scrape {
        clear_console();

        let company_key = company_option.to_string();

        // Set a message to show current activity
        pb.set_message(format!("Scraping {}", company_key));

        // Start timing the operation
        let start = Instant::now();

        // Perform the scraping
        let jobs_payload = match company_option.scrape_jobs(data).await {
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

            all_new_jobs.extend(jobs_payload.new_jobs.iter().map(|j| FormattedJob {
                display_name: format!("{} | {} | ({})", j.title, j.location, company_key),
                job: j.clone(),
                company: company_key.clone(),
            }));

            if data.smart_criteria_enabled {
                pb.println("🧠 Filtering jobs based on smart criteria");
                let openai_client = OpenAIClient::new();

                let filtered_jobs = openai_client
                    .filter_jobs_based_on_smart_criteria(&jobs_payload.new_jobs)
                    .await?;

                let formatted_jobs = filtered_jobs
                    .iter()
                    .map(|j| FormattedJob {
                        display_name: format!("{} | {} | ({})", j.title, j.location, company_key),
                        job: j.clone(),
                        company: company_key.clone(),
                    })
                    .collect::<Vec<FormattedJob>>();

                new_jobs_based_on_smart_criteria.extend(formatted_jobs);
            }
        }

        // Optional: Show time taken for each company
        let elapsed = start.elapsed();
        pb.set_message(format!("Done in {:.2}s", elapsed.as_secs_f64()));
    }

    // Finish the progress bar
    pb.finish_with_message("Scraping completed!");

    // If no new jobs found, return an error
    if all_new_jobs.is_empty() {
        return Err("No new jobs found across your network and followed companies".into());
    }

    // Create a new jobs report based on all new jobs
    create_report(&all_new_jobs, ReportMode::HTML)?;

    // Return the new jobs based on smart criteria if smart criteria is enabled
    if data.smart_criteria_enabled {
        Ok(new_jobs_based_on_smart_criteria)
    } else {
        Ok(all_new_jobs)
    }
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
    #[strum(to_string = "Manage Smart Criteria")]
    ManageSmartCriteria,
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
) -> AppResult<()> {
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
                    if let Some(company) = data.companies.get_mut(company_name) {
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

#[derive(Display, EnumIter, Clone)]
pub enum ManageSmartCriteriaOptions {
    #[strum(to_string = "Set Smart Criteria")]
    SetSmartCriteria,

    #[strum(to_string = "Enable Smart Criteria [ ]")]
    EnableSmartCriteria,

    #[strum(to_string = "Back")]
    Back,
}
pub fn prompt_user_for_manage_smart_criteria_selection() -> ManageSmartCriteriaOptions {
    let dialoguer_styles = ColorfulTheme::default();

    let data = Data::get_data();

    // Create vectors to store both the display strings and corresponding enum variants
    let mut display_strings = Vec::new();
    let mut variants = Vec::new();

    // Build the menu options based on state
    for variant in ManageSmartCriteriaOptions::iter() {
        match variant {
            ManageSmartCriteriaOptions::EnableSmartCriteria => {
                if !data.smart_criteria.is_empty() {
                    let display = if data.smart_criteria_enabled {
                        "Enable Smart Criteria [x]"
                    } else {
                        "Enable Smart Criteria [ ]"
                    };
                    display_strings.push(display.to_string());
                    variants.push(variant);
                }
            }
            _ => {
                display_strings.push(variant.to_string());
                variants.push(variant);
            }
        }
    }

    let idx = Select::with_theme(&dialoguer_styles)
        .with_prompt("Select an option")
        .items(&display_strings)
        .interact()
        .unwrap();

    variants[idx].clone()
}

pub fn handle_manage_smart_criteria() {
    loop {
        clear_console();
        let mut data = Data::get_data();

        println!();
        println!(
            "Smart Criteria: {}",
            if data.smart_criteria.is_empty() {
                "No smart criteria set".red()
            } else {
                data.smart_criteria.green()
            }
        );
        println!();
        let manage_smart_criteria_selection = prompt_user_for_manage_smart_criteria_selection();

        match manage_smart_criteria_selection {
            ManageSmartCriteriaOptions::SetSmartCriteria => {
                clear_console();
                let dialoguer_styles = ColorfulTheme::default();
                let smart_criteria = format!(
                    "I am interested in jobs that {}",
                    Input::<String>::with_theme(&dialoguer_styles)
                        .with_prompt("Enter your smart criteria")
                        .with_initial_text("I am interested in jobs that ")
                        .interact()
                        .unwrap()
                );

                data.set_smart_criteria(smart_criteria);
            }

            ManageSmartCriteriaOptions::EnableSmartCriteria => {
                println!("Smart Criteria Enabled");
                data.toggle_smart_criteria_enabled();
            }

            _ => break,
        }
    }
}

pub fn handle_view_new_jobs_reports() -> AppResult<()> {
    let v = Data::get_new_jobs_report_files();
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
