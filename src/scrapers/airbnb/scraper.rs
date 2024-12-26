use std::error::Error;

use headless_chrome::{Browser, LaunchOptions};

use crate::models::{
    data::Data,
    scraper::{JobsPayload, ScrapedJob},
};

pub async fn scrape_airbnb(data: &mut Data) -> Result<JobsPayload, Box<dyn Error>> {
    let launch_options = LaunchOptions {
        headless: true,
        window_size: Some((1920, 1080)),
        enable_logging: true,

        ..LaunchOptions::default()
    };
    let browser = Browser::new(launch_options)?;

    let tab = browser.new_tab()?;

    let mut paged = 1;

    let mut scraped_jobs: Vec<ScrapedJob> = Vec::new();

    loop {
        let url = format!(
            "https://careers.airbnb.com/positions/?_departments=engineering&paged={}",
            paged
        );
        tab.navigate_to(&url)?;
        tab.wait_for_element("body")?;
        tab.wait_for_element(".section-container")?;
        let remote_object = tab.evaluate(r#"
        const jobNodes = [...document.querySelectorAll("li[role='listitem']")];

const scrapedJobs = jobNodes.map(node => {
    const title = node.querySelector("h3").textContent;
    const location = node.querySelector("span.text-size-4.font-normal.text-gray-48.flex.items-center").textContent.trim();
    const link = node.querySelector("a").href;

    return {
        title,
        location,
        link
    }
})

JSON.stringify(scrapedJobs);


        "#, false)?;

        let scraped_jobs_subset: Vec<ScrapedJob> =
            serde_json::from_str(remote_object.value.unwrap().as_str().unwrap()).unwrap();

        if scraped_jobs_subset.is_empty() {
            break;
        }

        scraped_jobs.extend(scraped_jobs_subset);

        paged += 1;
    }
    // Acquire Vector of ScrapedJob

    // Convert Vector of ScrapedJob into a JobsPayload
    let jobs_payload = JobsPayload::from_scraped_jobs(scraped_jobs, "AirBnB", data);

    // Return JobsPayload
    Ok(jobs_payload)
}
