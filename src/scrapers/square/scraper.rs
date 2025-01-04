use std::error::Error;

use reqwest::Client;
use serde_json::Value;

use crate::{error::AppResult, models::{
    data::Data,
    scraper::{JobsPayload, ScrapedJob},
}};

pub async fn scrape_square(data: &mut Data) -> AppResult<JobsPayload> {
    let mut page = 1;
    let mut scraped_jobs: Vec<ScrapedJob> = Vec::new();

    loop {
        let url = format!("https://block.xyz/api/careers/jobs?businessUnits[]=square&page={}&pageLimit=10&teams[]=Software%20Engineering", page);

        let json = Client::new().get(url).send().await?.json::<Value>().await?;

        if let Some(jobs) = json["currentPage"].as_array() {
            if jobs.is_empty() {
                break;
            }

            let scraped_jobs_subset: Vec<ScrapedJob> = jobs
                .iter()
                .map(|v| ScrapedJob {
                    title: v["title"].as_str().unwrap().trim().to_string(),
                    location: v["location"].as_str().unwrap().trim().to_string(),
                    link: format!(
                        "https://block.xyz/careers/jobs/{}",
                        v["id"].as_i64().unwrap()
                    ),
                })
                .collect();

            scraped_jobs.extend(scraped_jobs_subset);
            page += 1;
        }
    }

    // Convert Vector of ScrapedJob into a JobsPayload
    let jobs_payload = JobsPayload::from_scraped_jobs(scraped_jobs, "Square", data);

    // Return JobsPayload
    Ok(jobs_payload)
}
