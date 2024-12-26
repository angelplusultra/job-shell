use std::{error::Error, time::Duration};

use dialoguer::Select;
use headless_chrome::{Browser, LaunchOptions};

use crate::models::{
    data::Data,
    scraper::{JobsPayload, ScrapedJob},
};

pub async fn scrape_gen(data: &mut Data) -> Result<JobsPayload, Box<dyn Error>> {
    let launch_options = LaunchOptions {
        headless: true,
        window_size: Some((1920, 1080)),
        enable_logging: true,

        ..LaunchOptions::default()
    };
    let browser = Browser::new(launch_options)?;

    let tab = browser.new_tab()?;

    tab.navigate_to("https://gen.wd1.myworkdayjobs.com/careers/?jobFamilyGroup=f0cfdff3f4311000b8ae5a80a71b0000")?;
    tab.wait_for_element("body")?;

    let mut next_button_result = tab.wait_for_element("button[aria-label='next']");

    let mut total_scraped_jobs: Vec<ScrapedJob> = Vec::new();
    while next_button_result.is_ok() {
        tab.wait_for_element("section[data-automation-id='jobResults']")?;
        // scrape jobs

        let remote_object = tab.evaluate(
            r#"
        
JSON.stringify([...document.querySelectorAll("li.css-1q2dra3")].map(el => {
    return {
        title: el.querySelector("a").innerText.trim(),
        link: el.querySelector("a").href.trim(),
        location: el.querySelector("dd.css-129m7dg").innerText.trim()
    }
}))

        "#,
            false,
        )?;

        let scraped_jobs: Vec<ScrapedJob> =
            serde_json::from_str(remote_object.value.unwrap().as_str().unwrap()).unwrap();

        for sc in scraped_jobs {
            total_scraped_jobs.push(sc);
        }

        if let Ok(next_button) = next_button_result {
            next_button.click()?;
            next_button_result = tab.wait_for_element_with_custom_timeout(
                "button[aria-label='next']",
                Duration::from_secs(3),
            );
        }
    }

    let jobs_payload = JobsPayload::from_scraped_jobs(total_scraped_jobs, "Gen", data);

    Ok(jobs_payload)
}
