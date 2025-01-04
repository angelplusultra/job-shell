use std::error::Error;

use serde_json::Value;

use crate::{error::AppResult, models::{
    data::Data,
    scraper::{JobsPayload, ScrapedJob},
}};

pub async fn scrape_robinhood(data: &mut Data) -> AppResult<JobsPayload> {
    let json: Value = reqwest::get("https://api.greenhouse.io/v1/boards/robinhood/jobs")
        .await?
        .json()
        .await?;

    let mut scraped_jobs: Vec<ScrapedJob> = Vec::new();

    let engineering_jobs = json["jobs"].as_array().unwrap().iter().filter(|json| {
        json["metadata"].as_array().unwrap()[0]["value"] == "ENGINEERING & SECURITY"
    });

    for job in engineering_jobs {
        let locations = job["location"]["name"]
            .as_str()
            .unwrap()
            .split(";")
            .collect::<Vec<&str>>();

        for loction in locations {
            scraped_jobs.push(ScrapedJob {
                title: job["title"].as_str().unwrap().trim().to_string(),
                location: loction.trim().to_string(),
                link: job["absolute_url"].as_str().unwrap().to_string(),
            });
        }
    }

    // Convert Vector of ScrapedJob into a JobsPayload
    let jobs_payload = JobsPayload::from_scraped_jobs(scraped_jobs, "Robinhood", data);

    // Return JobsPayload
    Ok(jobs_payload)
}
