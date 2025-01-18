use headless_chrome::{Browser, LaunchOptions};

use crate::{
    error::AppResult,
    handlers::scrape_options::DefaultJobScraperOptions,
    models::{
        data::Data,
        scraper::{JobsPayload, ScrapedJob},
    },
};

pub async fn default_scrape_jobs_handler(
    data: &mut Data,
    options: DefaultJobScraperOptions,
) -> AppResult<JobsPayload> {
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

    let scraped_jobs: Vec<ScrapedJob> = serde_json::from_str(
        engineering_jobs
            .value
            .ok_or("No value returned from JavaScript evaluation")?
            .as_str()
            .ok_or("Value is not a string")?,
    )?;

    let jobs_payload = JobsPayload::from_scraped_jobs(scraped_jobs, options.company_key, data);

    Ok(jobs_payload)
}
