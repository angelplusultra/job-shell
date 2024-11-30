use chrono::Utc;
use clipboard::{ClipboardContext, ClipboardProvider};
use colored::*;
use scrapers::cisco::scraper::scrape_cisco;
use core::panic;
use dialoguer::theme::ColorfulTheme;
use dialoguer::{Confirm, Editor, FuzzySelect, Input, Select};
use dotenv::dotenv;
use handlers::handlers::{
    default_scrape_jobs_handler, handle_craft_a_message, handle_reach_out_to_a_connection,
    prompt_user_for_company_option, prompt_user_for_company_selection,
    prompt_user_for_connection_option, prompt_user_for_connection_selection,
    prompt_user_for_job_option, prompt_user_for_job_selection, JobOption,
    ReachOutToAConnectionOption,
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
use scrapers::blizzard::scraper::scrape_blizzard;
use scrapers::chase::scraper::scrape_chase;
use scrapers::coinbase::scraper::scrape_coinbase;
use scrapers::disney::scraper::scrape_disney;
use scrapers::gen::scraper::scrape_gen;
use scrapers::ibm::scraper::scrape_ibm;
use scrapers::meta::scraper::scrape_meta;
use scrapers::netflix::scraper::scrape_netflix;
use scrapers::reddit::scraper::scrape_reddit;
use scrapers::square::scraper::scrape_square;
use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::PathBuf;
use std::str::FromStr;
use std::thread::sleep;
use std::time::Duration;
use std::{env, fs};
use tabled::{
    settings::{Modify, Style, Width},
    Table,
};

use tokio::time::Instant;
use webbrowser;

// TODO: Keys should prob be lowercase, make a tuple where 0 is key and 1 is display name
const COMPANYKEYS: [&str; 19] = [
    "Anduril",
    "Blizzard",
    "Cisco",
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

        // let counts = data.get_job_counts();
        //
        //
        // println!("{:#?}", counts);
        // sleep(Duration::from_secs(10));

        enum MainMenuOptions {
            SelectACompany,
            ScrapeNewJobsAcrossNetwork,
            MyConnections,
            Exit,
        }

        trait IOptions {
            fn display_strings(&self) -> Vec<&str>;
        }

        impl IOptions for [(MainMenuOptions, &str)] {
            fn display_strings(&self) -> Vec<&str> {
                self.iter().map(|o| o.1).collect()
            }
        }
        let main_menu_options = [
            (MainMenuOptions::SelectACompany, "Select a Company"),
            (
                MainMenuOptions::ScrapeNewJobsAcrossNetwork,
                "Scrape New Jobs Across Network",
            ),
            (MainMenuOptions::MyConnections, "My Connections"),
            (MainMenuOptions::Exit, "Exit"),
        ];

        let main_menu_selection = &main_menu_options[Select::with_theme(&dialoguer_styles)
            .with_prompt("Select an option")
            .items(&main_menu_options.display_strings())
            .interact()
            .unwrap()];

        match main_menu_selection.0 {
            MainMenuOptions::SelectACompany => {
                let company_selection = prompt_user_for_company_selection();

                if company_selection == "Back" {
                    continue;
                }

                let company = company_selection;

                //INFO: Company Loop
                loop {
                    let selected_company_option = prompt_user_for_company_option(company);

                    match selected_company_option {
                        "Back" => break,
                        "View Jobs" => {
                            if data.data[company].jobs.is_empty() {
                                eprintln!("Error: No jobs");
                                continue;
                            }
                            let jobs = data.data.get(company).unwrap().jobs.clone();
                            //

                            match prompt_user_for_job_selection(jobs, None) {
                                Some(selected_job) => {
                                    data.mark_job_seen(&selected_job.id);
                                }
                                None => break,
                            }
                        }
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

                                        loop {
                                            let prompt = format!(
                                                "Select an option for {}",
                                                selected_job.title
                                            );

                                            let job_options = [
                                                "Open Job in Browser",
                                                "Reach Out to a Connection",
                                                "Generate Job Details with AI",
                                                "Back",
                                            ];
                                            let job_options_selection = job_options
                                                [Select::with_theme(&dialoguer_styles)
                                                    .with_prompt(prompt)
                                                    .items(&job_options)
                                                    .interact()
                                                    .unwrap()];

                                            match job_options_selection {
                                                "Open Job in Browser" => {
                                                    webbrowser::open(&selected_job.link)?;

                                                    let apply =
                                                        Confirm::with_theme(&dialoguer_styles)
                                                            .with_prompt("Did you apply?")
                                                            .interact()
                                                            .unwrap();

                                                    if apply {
                                                        if let Some(company) =
                                                            data.data.get_mut(company)
                                                        {
                                                            //TODO: search by ID field when added to struct
                                                            let selected_job = company
                                                                .jobs
                                                                .iter_mut()
                                                                .find(|j| j.id == selected_job.id)
                                                                .unwrap();
                                                            selected_job.applied = true;

                                                            data.save();
                                                        }
                                                    }

                                                    continue;
                                                }
                                                "Reach Out to a Connection" => {
                                                    let break_loop =
                                                        handle_reach_out_to_a_connection(
                                                            &data.data[company].connections,
                                                            &selected_job,
                                                        )?;

                                                    if break_loop {
                                                        break;
                                                    }
                                                }
                                                "Generate Job Details with AI" => {
                                                    let job_details = match company {
                            // "Weedmaps" => get_weedmaps_jod_details(&selected_job).await?,
                            "1Password" => default_get_job_details(&selected_job, true, "body").await?,
                            "Tarro" => {
                                default_get_job_details(&selected_job, true, "._content_ud4nd_71").await?
                            }
                            "Discord" => default_get_job_details(&selected_job, true, "body").await?,
                            "Palantir" => default_get_job_details(&selected_job, true, ".content").await?,
                            "Anduril" => default_get_job_details(&selected_job, true, "main").await?,
                            "Coinbase" => default_get_job_details(&selected_job, false, ".Flex-sc-9cfb0d13-0.Listing__Container-sc-bcedfe82-0.fXHNQM.dBburU").await?,
                            _ => default_get_job_details(&selected_job, true, "body").await?,
                        };

                                                    // Print details
                                                    // clear_console();
                                                    job_details.print_job();
                                                }
                                                _ => break,
                                            }
                                        }
                                    }
                                    None => break,
                                }
                                // INFO: Mark Job as seen
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
            MainMenuOptions::ScrapeNewJobsAcrossNetwork => {
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

                for company_key in companies_to_scrape {
                    clear_console();

                    // Set a message to show current activity
                    pb.set_message(format!("Scraping {}", company_key));

                    // Start timing the operation
                    let start = Instant::now();

                    // Perform the scraping
                    let jobs_payload = match scrape_jobs(&mut data, &company_key).await {
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

                create_report(&new_jobs, ReportMode::HTML)?;

                loop {
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

                    match prompt_user_for_job_option(&selected_job.job).0 {
                        JobOption::OpenJobInBrowser => {
                            webbrowser::open(&selected_job.job.link)?;
                            let did_apply = prompt_user_did_apply();

                            if did_apply {
                                data.mark_job_applied(&selected_job.job.id);
                            }
                        }
                        JobOption::ReachOut => {}
                        JobOption::GenerateJobDetails => {}
                        JobOption::Back => continue,
                    }
                }
            }
            MainMenuOptions::MyConnections => {
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
    let jobs_payload = match company_key {
        "Anduril" => default_scrape_jobs_handler(data, ANDURIL_SCRAPE_OPTIONS).await,
        "Chase" => scrape_chase(data).await,
        "Cisco" => scrape_cisco(data).await,
        "Blizzard" => scrape_blizzard(data).await,
        "Coinbase" => scrape_coinbase(data).await,
        "Weedmaps" => default_scrape_jobs_handler(data, WEEDMAPS_SCRAPE_OPTIONS).await,
        "1Password" => default_scrape_jobs_handler(data, ONEPASSWORD_SCRAPE_OPTIONS).await,

        "Discord" => default_scrape_jobs_handler(data, DISCORD_SCRAPE_OPTIONS).await,
        "Palantir" => default_scrape_jobs_handler(data, PALANTIR_DEFAULT_SCRAPE_OPTIONS).await,
        "Reddit" => scrape_reddit(data).await,
        "Gen" => scrape_gen(data).await,
        "IBM" => scrape_ibm(data).await,
        "Disney" => scrape_disney(data).await,
        "Meta" => scrape_meta(data).await,
        "Netflix" => scrape_netflix(data).await,
        "Square" => scrape_square(data).await,

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
struct FormattedJob {
    company: String,
    display_name: String,
    job: Job,
}
fn create_report(new_jobs: &Vec<FormattedJob>, mode: ReportMode) -> Result<(), Box<dyn Error>> {
    let today = Utc::now().naive_utc().date().to_string();

    let mut path = PathBuf::new();

    if cfg!(test) {
        path.push("tests");
        if !fs::exists(&path)? {
            fs::create_dir(&path)?;
        }
    }
    path.push("reports");

    if !fs::exists(&path)? {
        fs::create_dir(&path)?;
    }
    match mode {
        ReportMode::CSV => {
            let names_row = "Company,Title,Location,Link\n";
            let entries = new_jobs
                .iter()
                .map(|j| {
                    format!(
                        "{},{},{},{}\n",
                        j.company,
                        j.job.title,
                        j.job.location.replace(",", ""),
                        j.job.link
                    )
                })
                .collect::<String>();
            let csv = format!("{}{}", names_row, entries);
            // check if the root path exists
            path.push(today + ".csv");

            if fs::exists(&path)? {
                let mut file = OpenOptions::new().append(true).open(&path)?;

                write!(file, "{}", entries)?;
            } else {
                fs::write(&path, format!("{}", csv))?;
            }
        }
        ReportMode::HTML => {
            let html = format!(
                r#"
<!doctype html>
<html lang="en">
  <head>
    <meta charset="UTF-8" />
    <meta name="viewport" content="width=device-width, initial-scale=1" />
    <title></title>
  </head>

  <body>
    <h1>{}</h1>
    <table>
      <thead>
        <tr>
          <th>Company</th>
          <th>Title</th>
          <th>Location</th>
          <th>Link</th>
        </tr>
      </thead>

      <tbody>
        {}
      </tbody>
    </table>
  </body>
</html>
"#,
                format!("New Jobs: {}", today),
                new_jobs
                    .iter()
                    .map(|fj| {
                        format!(
                            r#"<tr><td>{}</td> <td>{}</td> <td>{}</td> <td><a href="{}">Apply</a></td></tr>"#,
                            fj.company, fj.job.title, fj.job.location, fj.job.link
                        )
                    })
                    .collect::<String>()
            );

            path.push(today + ".html");

            fs::write(&path, html)?;
        }
    }

    Ok(())
}

enum ReportMode {
    HTML,
    CSV,
}

#[cfg(test)]
mod test {
    use uuid::Uuid;

    use super::*;

    #[test]
    fn test_create_report() {
        let v = create_report(
            &vec![FormattedJob {
                display_name: "SWE".to_string(),
                company: "Disney".to_string(),
                job: Job {
                    title: "Software Engineer".to_string(),
                    link: "https://somelinktoapply.com".to_string(),
                    location: "Anaheim, CA".to_string(),
                    id: Uuid::new_v4(),
                    is_seen: false,
                    applied: false,
                },
            }],
            ReportMode::HTML,
        );

        assert_eq!(v.is_ok(), true);
    }
}
