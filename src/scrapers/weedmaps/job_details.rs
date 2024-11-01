use std::error::Error;

use headless_chrome::{Browser, LaunchOptions};
use reqwest::Client;
use serde_json::json;

use crate::models::{self, gemini::GeminiJob, scraper::Job};

pub async fn get_weedmaps_jod_details(job: &Job) -> Result<GeminiJob, Box<dyn Error>> {
    let options = LaunchOptions {
        headless: false,
        window_size: Some((1920, 1080)),
        enable_logging: true,

        ..LaunchOptions::default()
    };
    let browser = Browser::new(options)?;
    let tab = browser.new_tab()?;

    tab.navigate_to(&job.link)?;

    tab.wait_for_element("body")?;
    tab.wait_for_element("#grnhse_iframe")?;

    let iframe = tab.evaluate("document.querySelector('#grnhse_iframe').src", false)?;

    let apply_link = iframe.value.unwrap();

    let href = apply_link.as_str().unwrap();

    tab.navigate_to(href)?;

    let job_description = tab.evaluate("document.querySelector('#content').innerHTML", false)?;


    if let Some(html) = job_description.value {
        let weedmaps_gemini_job = GeminiJob::from_job_html(html.to_string()).await?;

        return Ok(weedmaps_gemini_job);
    } else {
        return Err(Box::new(models::custom_error::CustomError {
            details: "An error occured scraping job details content".to_string(),
        }));
    }
}
