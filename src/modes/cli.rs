use std::{error::Error, thread::sleep, time::Duration};

use colored::Colorize;
use dialoguer::{theme::ColorfulTheme, Confirm, FuzzySelect, Input};
use indicatif::{ProgressBar, ProgressStyle};
use jobshell::utils::{clear_console, stall_and_present_countdown};
use tabled::{settings::Style, Table, Tabled};

use crate::{
    company_options::ScrapeJobs, error::AppResult, handle_view_new_jobs_reports, handlers::handlers::{
        handle_job_selection, handle_manage_connection, handle_manage_smart_criteria,
        handle_open_job_in_browser, handle_reach_out_to_a_connection,
        handle_scan_new_jobs_across_network_and_followed_companies, prompt_user_for_company_option,
        prompt_user_for_company_selection_v2, prompt_user_for_job_option,
        prompt_user_for_main_menu_selection, FormattedJob, JobOption, MainMenuOption,
        SelectedCompanyOption,
    }, models::{
        data::{Connection, Data},
        scraper::{Job, JobsPayload},
    }
};

pub async fn run() -> AppResult<()> {
    let dialoguer_styles = ColorfulTheme::default();

    let font_data = include_str!("../fonts/slant.flf");
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
                    // let company_selection = prompt_user_for_company_selection();

                    let selected_company_opt = prompt_user_for_company_selection_v2();

                    if let None = selected_company_opt {
                        break;
                    }

                    let selected_company = selected_company_opt.unwrap();

                    let company_string = selected_company.to_string();

                    let company = company_string.as_str();
                    //INFO: Company Loop
                    loop {
                        clear_console();
                        let is_following = data.companies[company].is_following;
                        let selected_company_option =
                            prompt_user_for_company_option(company, is_following);

                        match selected_company_option {
                            SelectedCompanyOption::Back => break,
                            SelectedCompanyOption::ViewJobs => {
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
                            SelectedCompanyOption::ViewOrEditConnections => {
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
                            SelectedCompanyOption::ScrapeAndUpdateJobs => {
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
                                } = match selected_company.scrape_jobs(&mut data).await {
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
                            SelectedCompanyOption::AddAConnection => {
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
                            SelectedCompanyOption::FollowCompany => {
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

async fn handle_job_option(
    selected_job: &Job,
    data: &mut Data,
    company: &str,
) -> AppResult<()> {
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
