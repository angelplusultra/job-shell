use std::error::Error;

use dialoguer::{theme::ColorfulTheme, FuzzySelect, Select};
use headless_chrome::{Browser, LaunchOptions};

use crate::{
    models::{
        data::Data,
        scraper::{JobsPayload, ScrapedJob},
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
        "Back",
    ];

    let selection = Select::with_theme(&dialoguer_styles)
        .with_prompt(&format!("Select an option for {}", company))
        .items(&options)
        .interact()
        .unwrap();

    return options[selection];
}
