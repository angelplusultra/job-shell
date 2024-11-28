use std::error::Error;

use headless_chrome::{Browser, LaunchOptions};
use reqwest::Client;
use serde_json::Value;

use crate::models::{
    data::Data,
    scraper::{Job, JobsPayload, ScrapedJob},
};

pub async fn scrape_netflix(data: &mut Data) -> Result<JobsPayload, Box<dyn Error>> {
    let mut i = 0;
    let mut positions: Vec<ScrapedJob> = Vec::new();
    loop {
        let url = format!(
            "https://explore.jobs.netflix.net/api/apply/v2/jobs?&start={}&num=10&Teams=Engineering&Teams=Engineering%20Operations&Teams=Data%20Science%20%26%20Analytics",
            i
        );
        let json = Client::new().get(url).send().await?.json::<Value>().await?;

        if json["positions"].as_array().unwrap().is_empty() {
            break;
        }

        let scraped_jobs: Vec<ScrapedJob> = json["positions"]
            .as_array()
            .unwrap()
            .iter()
            .map(|v| ScrapedJob {
                title: v["name"].as_str().unwrap().trim().to_string(),
                location: v["location"].as_str().unwrap().trim().to_string(),
                link: v["canonicalPositionUrl"].as_str().unwrap().trim().to_string(),
            })
            .collect();

        positions.extend(scraped_jobs);

        i += 10;
    }
    let jobs_payload = JobsPayload::from_scraped_jobs(positions, &data.data["Netflix"]);

    data.data.get_mut("Netflix").unwrap().jobs = jobs_payload.all_jobs.clone();

    data.save();

    Ok(jobs_payload)
}
