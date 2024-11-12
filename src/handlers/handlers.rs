use std::error::Error;

use headless_chrome::{Browser, LaunchOptions};

use crate::models::{
    data::Data,
    scraper::{JobsPayload, ScrapedJob},
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

    let onepassword_jobs_payload =
        JobsPayload::from_scraped_jobs(scraped_jobs, &data.data[options.company_key]);

    data.data.get_mut(options.company_key).unwrap().jobs =
        onepassword_jobs_payload.all_jobs.clone();

    data.save();

    Ok(onepassword_jobs_payload)
}
