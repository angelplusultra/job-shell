use std::error::Error;

use headless_chrome::{Browser, LaunchOptions};

use crate::models::{
    data::Data,
    scraper::{JobsPayload, ScrapedJob},
};

pub async fn scrape_salesforce(data: &mut Data) -> Result<JobsPayload, Box<dyn Error>> {
    let launch_options = LaunchOptions {
        headless: false,
        window_size: Some((1920, 1080)),
        enable_logging: true,

        ..LaunchOptions::default()
    };
    let browser = Browser::new(launch_options)?;

    let tab = browser.new_tab()?;
    let mut page = 1;
    let mut scraped_jobs: Vec<ScrapedJob> = Vec::new();
    loop {
        let url = format!("https://careers.salesforce.com/en/jobs/?page={page}&team=Software%20Engineering&pagesize=50#results",);
        tab.navigate_to(&url)?;
        tab.wait_for_element("body")?;

        let remote_object = tab.evaluate(
            r##"

const jobCards = [...document.querySelectorAll(".card.card-job")].map(node => {
    const title = node.querySelector(".card-title").textContent.trim();

    // Clean and join locations
    const location = Array.from(node.querySelectorAll(".locations"))
        .map(locNode => locNode.textContent.trim().replaceAll("\n", "").replaceAll("\t", " "))
        .map(text => text.split(" ").filter(w => w.trim() !== "").join(" "))
        .join(" | ");

    const link = node.querySelector("a").href;
    
    return {
        title,
        location,
        link
    };
});

JSON.stringify(jobCards);
    "##,
            false,
        )?;

        let results = remote_object.value.as_ref().unwrap();

        // TODO: Fix this eventually
        if results.as_str().unwrap() == "[]" {
            break;
        }

        let scraped_jobs_subset: Vec<ScrapedJob> = serde_json::from_str(results.as_str().unwrap())?;

        scraped_jobs.extend(scraped_jobs_subset);

        page += 1;
    }

    // Convert Vector of ScrapedJob into a JobsPayload
    let jobs_payload = JobsPayload::from_scraped_jobs(scraped_jobs, &data.data["Salesforce"]);

    // REMEBER TO SAVE THE NEW JOBS TO THE DATA STATE
    data.data.get_mut("Salesforce").unwrap().jobs = jobs_payload.all_jobs.clone();
    data.save();

    // Return JobsPayload
    Ok(jobs_payload)
}

