use std::{error::Error, path::PathBuf};

use headless_chrome::{Browser, LaunchOptions};

use crate::{
    models::{
        scraper::{Job, JobsPayload},
        snapshots::Snapshots,
    },
    utils::stringify_js::strinfify_js,
};

pub async fn scrape_anduril(snapshots: &mut Snapshots) -> Result<JobsPayload, Box<dyn Error>> {
    let mut file_path = PathBuf::from(file!());
    file_path.pop();

    file_path.push("scripts/get_anduril_jobs.js");

    let js = strinfify_js(file_path)?;

    let launch_options = LaunchOptions {
        headless: false,
        ..LaunchOptions::default()
    };

    let browser = Browser::new(launch_options)?;

    let tab = browser.new_tab()?;

    tab.navigate_to("https://www.anduril.com/open-roles")?;

    tab.wait_until_navigated()?;

    let remote_object = tab.evaluate(&js, false)?;

    let json_jobs = remote_object.value.unwrap();

    let jobs: Vec<Job> = serde_json::from_str(json_jobs.as_str().unwrap()).unwrap();

    let jobs_payload = JobsPayload::from_jobs(&jobs, &snapshots.anduril);

    snapshots.anduril = jobs;
    snapshots.save();

    Ok(jobs_payload)
}
