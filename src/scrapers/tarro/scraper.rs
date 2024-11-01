use core::panic;
use std::{error::Error, path::PathBuf, thread::sleep, time::Duration};

use headless_chrome::{
    protocol::cdp::Animation::ReleaseAnimationsReturnObject, Browser, LaunchOptions,
};
use reqwest::Client;
use serde_json::{json, Value};

use crate::{
    models::{
        gemini::{GeminiClient, Root},
        scraper::{Job, JobsPayload},
        snapshots::Snapshots,
    },
    utils::stringify_js::strinfify_js,
};

pub async fn scrape_tarro(snapshots: &mut Snapshots) -> Result<JobsPayload, Box<dyn Error>> {
    let mut file_path = PathBuf::from(file!());

    file_path.pop();

    file_path.push("scripts/get_tarro_jobs.js");

    let js = strinfify_js(file_path)?;

    let options = LaunchOptions {
        headless: true,
        window_size: Some((1920, 1080)),
        enable_logging: true,

        ..LaunchOptions::default()
    };

    let browser = Browser::new(options)?;

    let tab = browser.new_tab()?;

    tab.navigate_to("https://www.tarro.com/about-us/careers#jobboard")?;

    tab.wait_until_navigated()?;

    tab.wait_for_element("body")?;
    tab.wait_for_element("#departments-container")?;

    sleep(Duration::from_secs(2));
    let json_jobs = tab.evaluate(&js, false)?;



    let jobs: Vec<Job> = serde_json::from_str(json_jobs.value.unwrap().as_str().unwrap()).unwrap();

    let jobs_payload = JobsPayload::from_jobs(&jobs, &snapshots.tarro);

    snapshots.tarro = jobs;
    snapshots.save();

    Ok(jobs_payload)
}
