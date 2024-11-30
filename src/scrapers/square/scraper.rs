use std::error::Error;

use reqwest::Client;
use serde_json::Value;

use crate::models::{
    data::Data,
    scraper::{JobsPayload, ScrapedJob},
};

pub async fn scrape_square(data: &mut Data) -> Result<JobsPayload, Box<dyn Error>> {
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
    let jobs_payload = JobsPayload::from_scraped_jobs(scraped_jobs, &data.data["Square"]);

    // REMEBER TO SAVE THE NEW JOBS TO THE DATA STATE
    data.data.get_mut("Square").unwrap().jobs = jobs_payload.all_jobs.clone();
    data.save();

    // Return JobsPayload
    Ok(jobs_payload)
}

