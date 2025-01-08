use std::error::Error;

use headless_chrome::{Browser, LaunchOptions};

use crate::{error::AppResult, models::{
    data::Data,
    scraper::{JobsPayload, ScrapedJob},
}};

pub async fn scrape_doordash(data: &mut Data) -> AppResult<JobsPayload> {
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
        tab.navigate_to(&format!(
            "https://careersatdoordash.com/job-search/?department=Engineering%7C&intern=0&spage={}",
            page
        ))?;
        tab.wait_for_element("body")?;
        tab.wait_for_element(".container")?;

        let remote_object = tab.evaluate(
            r#"
            const jobItems = [...document.querySelectorAll(".job-item")].flatMap(node => {
    const titleCont = node.querySelector(".title-container");

    const titleAndLink = titleCont.querySelector("a");

    const locCont = node.querySelector(".location-container");

    const locations = locCont.querySelector(".value-secondary").innerText.split(";");

    const jobsBatch = []
    for(const location of locations) {
    if(location === "") {
    continue
}
        jobsBatch.push({
            title: titleAndLink.innerText.trim(),
    link: titleAndLink.href,
location: location.trim()
        })
    }
    
    

return jobsBatch
});


JSON.stringify(jobItems);
            "#,
            false,
        )?;

        let scraped_jobs_batch: Vec<ScrapedJob> =
            serde_json::from_str(remote_object.value.unwrap().as_str().unwrap())?;

        if scraped_jobs_batch.is_empty() {
            break;
        }

        scraped_jobs.extend(scraped_jobs_batch);

        page += 1;
    }

    // Convert Vector of ScrapedJob into a JobsPayload
    let jobs_payload = JobsPayload::from_scraped_jobs(scraped_jobs, "DoorDash", data);

    // Return JobsPayload
    Ok(jobs_payload)
}

