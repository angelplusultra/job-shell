use std::error::Error;

use serde_json::Value;

use crate::{
    error::AppResult,
    models::{
        data::Data,
        scraper::{JobsPayload, ScrapedJob},
    },
};

pub async fn scrape_panasonic(data: &mut Data) -> AppResult<JobsPayload> {
    let mut page = 1;

    let mut scraped_jobs = Vec::new();
    loop {
        let api_url =
            format!("https://careers.na.panasonic.com/api/jobs?page={page}&categories=Engineering");

        let json: Value = reqwest::get(&api_url).await?.json().await?;

        let jobs: Vec<Value> = json["jobs"].as_array().cloned().unwrap();

        if jobs.is_empty() {
            break;
        }

        let scarped_jobs_batch: Vec<ScrapedJob> = jobs
            .iter()
            .map(|j| ScrapedJob {
                title: j["data"]["title"].as_str().unwrap().to_string(),
                location: format!(
                    "{}, {}",
                    j["data"]["city"].as_str().unwrap(),
                    j["data"]["country"].as_str().unwrap()
                ),
                link: j["data"]["meta_data"]["canonical_url"]
                    .as_str()
                    .unwrap()
                    .to_string(),
            })
            .collect();

        scraped_jobs.extend(scarped_jobs_batch);

        page += 1;
    }

    let jobs_payload = JobsPayload::from_scraped_jobs(scraped_jobs, "Panasonic", data);
    // Return JobsPayload
    Ok(jobs_payload)
}

