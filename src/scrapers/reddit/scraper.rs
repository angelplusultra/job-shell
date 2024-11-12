use std::error::Error;

use headless_chrome::{Browser, LaunchOptions};

use crate::models::{
    data::Data,
    scraper::{JobsPayload, ScrapedJob},
};

pub async fn scrape_reddit(data: &mut Data) -> Result<JobsPayload, Box<dyn Error>> {
    let launch_options = LaunchOptions {
        headless: false,
        window_size: Some((1920, 1080)),
        enable_logging: true,

        ..LaunchOptions::default()
    };
    let browser = Browser::new(launch_options)?;

    let tab = browser.new_tab()?;

    tab.navigate_to("https://redditinc.com/careers")?;
    tab.wait_for_element("body")?;

    let jobs = tab.find_element("div[data-departmentid='16253']")?;

    jobs.click()?;
    let scraped_jobs = tab.evaluate(
        r##"

// DELETE AND REPLACE WITH CUSTOM JS LOGIC    
const engJobs = document.querySelector("#jobs-16253")

const jobsPayload = Array.from(engJobs.querySelectorAll(".job")).map(j => {
    const title = j.querySelector(".job-title").innerHTML;
    const location = j.querySelector(".job-location").innerHTML;
    const link = j.querySelector("a").href;

    return {
        title,
        location,
        link
    }
})

JSON.stringify(jobsPayload);
    "##,
        false,
    )?;

    let scraped_jobs: Vec<ScrapedJob> =
        serde_json::from_str(scraped_jobs.value.unwrap().as_str().unwrap()).unwrap();

    let jobs_payload = JobsPayload::from_scraped_jobs(scraped_jobs, &data.data["Reddit"]);

    data.data.get_mut("Reddit").unwrap().jobs = jobs_payload.all_jobs.clone();

    data.save();

    Ok(jobs_payload)
}

