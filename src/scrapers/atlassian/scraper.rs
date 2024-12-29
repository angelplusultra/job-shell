use std::error::Error;

use reqwest::Client;
use serde_json::Value;

use crate::models::{
    data::Data,
    scraper::{JobsPayload, ScrapedJob},
};

pub async fn scrape_atlassian(data: &mut Data) -> Result<JobsPayload, Box<dyn Error>> {
    let client = Client::new();
    let json: Value = client
    .get("https://www.atlassian.com/endpoint/careers/listings")
    .header("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36")
    .send()
    .await?
    .json()
    .await?;

    let jobs_of_interest = json.as_array().unwrap().iter().filter(|&j| {
        let category = j["category"].as_str().unwrap();
        if category == "Engineering" || category == "Interns" || category == "Graduates" {
            return true;
        } else {
            return false;
        }
    });

    let mut scraped_jobs: Vec<ScrapedJob> = Vec::new();

    for job in jobs_of_interest {
        let locations = job["locations"].as_array().unwrap();

        for location in locations {
            let title = job["title"].as_str().unwrap().trim().to_string();
            let formatted_location = location
                .as_str()
                .unwrap()
                .split("-")
                .take(2)
                .map(|s| s.trim())
                .collect::<Vec<&str>>()
                .join(", ");

            let link = job["portalJobPost"]["portalUrl"]
                .as_str()
                .unwrap()
                .to_string();

            let scraped_job = ScrapedJob {
                title,
                location: formatted_location,
                link,
            };

            scraped_jobs.push(scraped_job);
        }
    }

    let jobs_payload = JobsPayload::from_scraped_jobs(scraped_jobs, "Atlassian", data);

    // Return JobsPayload
    Ok(jobs_payload)
}
