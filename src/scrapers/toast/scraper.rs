use std::error::Error;

use headless_chrome::{Browser, LaunchOptions};

use crate::models::{
    data::Data,
    scraper::{JobsPayload, ScrapedJob},
};

pub async fn scrape_toast(data: &mut Data) -> Result<JobsPayload, Box<dyn Error>> {
    let launch_options = LaunchOptions {
        headless: true,
        window_size: Some((1920, 1080)),
        enable_logging: true,

        ..LaunchOptions::default()
    };
    let browser = Browser::new(launch_options)?;

    let tab = browser.new_tab()?;

    let mut page = 1;
    let mut scraped_jobs: Vec<ScrapedJob> = Vec::new();

    loop {
        let url = format!(
            "https://careers.toasttab.com/jobs/search?page={}&department_uids%5B%5D=546da8e254b79111ee592914ea196336&query=",
            page
        );
        tab.navigate_to(&url)?;
        tab.wait_for_element("body")?;
        tab.wait_for_element(".job-search-results-table")?;

        let remote_object = tab.evaluate(
            r##"

const cards = [...document.querySelectorAll(".job-search-results-card")]

const scrapedJobs = cards.map(card => {
    const title = card.querySelector(".card-title").innerText.trim();
    const link  =card.querySelector("a").href;

    const location = (card.querySelector(".job-component-location")?.innerText ?? "Anywhere").trim()
    
    return {
        title,
        link,
        location
        
    }
})

JSON.stringify(scrapedJobs)
    "##,
            false,
        )?;

        let scraped_jobs_batch: Vec<ScrapedJob> =
            serde_json::from_str(remote_object.value.unwrap().as_str().unwrap()).unwrap();

        if scraped_jobs_batch.is_empty() {
            break;
        }

        scraped_jobs.extend(scraped_jobs_batch);

        page += 1;
    }

    // Convert Vector of ScrapedJob into a JobsPayload
    let jobs_payload = JobsPayload::from_scraped_jobs(scraped_jobs, "Toast", data);

    // Return JobsPayload
    Ok(jobs_payload)
}
