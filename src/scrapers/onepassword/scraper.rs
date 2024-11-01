use std::{error::Error, path::PathBuf};

use headless_chrome::{Browser, LaunchOptions};

use crate::{
    models::scraper::{Job, JobsPayload},
    utils::stringify_js::strinfify_js,
    Snapshots,
};

pub async fn scrape_1password(snapshots: &mut Snapshots) -> Result<JobsPayload, Box<dyn Error>> {
    let mut file_path = PathBuf::from(file!());

    file_path.pop();

    file_path.push("scripts/get_onepassword_jobs.js");

    let js = strinfify_js(file_path)?;

    let options = LaunchOptions {
        headless: true,
        window_size: Some((1920, 1080)),
        enable_logging: true,

        ..LaunchOptions::default()
    };
    let browser = Browser::new(options)?;

    let tab = browser.new_tab()?;

    tab.navigate_to("https://jobs.lever.co/1password")?;
    tab.wait_for_element("body")?;
    tab.wait_for_element(".content")?;

    let engineering_jobs = tab.evaluate(&js, false)?;

    let jobs: Vec<Job> =
        serde_json::from_str(engineering_jobs.value.unwrap().as_str().unwrap()).unwrap();

    let onepassword_jobs_payload = JobsPayload::from_jobs(&jobs, &snapshots.onepassword);

    snapshots.onepassword = jobs;

    snapshots.save();

    Ok(onepassword_jobs_payload)
}
