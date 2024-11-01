use std::path::PathBuf;
use std::{collections::HashSet, error::Error, fs};

use crate::utils::stringify_js;
use crate::Snapshots;
use crate::{
    models::scraper::{Job, JobsPayload},
    utils::{snapshots::write_to_snapshots, stringify_js::strinfify_js},
};
use headless_chrome::{Browser, LaunchOptions};

pub async fn scrape_weedmaps(snapshots: &mut Snapshots) -> Result<JobsPayload, Box<dyn Error>> {
    let mut file_path = PathBuf::from(file!());

    file_path.pop();

    file_path.push("scripts/get_weedmaps_jobs.js");

    // TODO: Rename function
    let js = strinfify_js(file_path)?;

    let options = LaunchOptions {
        headless: false,
        window_size: Some((1920, 1080)),
        enable_logging: true,

        ..LaunchOptions::default()
    };

    let browser = Browser::new(options)?;

    let tab = browser.new_tab()?;

    tab.navigate_to("https://boards.greenhouse.io/embed/job_board?for=weedmaps77&b=https%3A%2F%2Fweedmaps.com%2Fcareers")?;

    // Wait for page to load (wait for body element)
    tab.wait_for_element("body")?;

    // get weedmaps software jobs
    let links = tab.evaluate(&js, false)?;

    let scraped_jobs: Vec<Job> = serde_json::from_str(links.value.unwrap().as_str().unwrap())?;

    let weedmaps_jobs_payload = JobsPayload::from_jobs(&scraped_jobs, &snapshots.weedmaps);

    snapshots.weedmaps = scraped_jobs;
    snapshots.save();

    Ok(weedmaps_jobs_payload)
}
