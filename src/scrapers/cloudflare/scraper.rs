use std::error::Error;

use reqwest::Client;
use serde_json::Value;

use crate::{error::AppResult, models::{
    data::Data,
    scraper::{JobsPayload, ScrapedJob},
}};

pub async fn scrape_cloudflare(data: &mut Data) -> AppResult<JobsPayload> {
    let mut scraped_jobs: Vec<ScrapedJob> = Vec::new();

    let mut json: Value = Client::new()
        .get("https://boards-api.greenhouse.io/v1/boards/cloudflare/departments/?render_as=tree")
        .send()
        .await?
        .json()
        .await?;

    let departments = json["departments"].as_array_mut().unwrap();

    let engineering_dep_id = 29067;
    let emerging_technologies_dep_id = 39629;
    departments.retain(|dep| {
        dep["id"].as_i64().unwrap() == engineering_dep_id
            || dep["id"].as_i64().unwrap() == emerging_technologies_dep_id
    });

    for dep in departments {
        for job in dep["jobs"].as_array().unwrap() {
            let metadata = job["metadata"].as_array().unwrap();

            let locations = metadata
                .iter()
                .find(|m| m["name"] == "Job Posting Location")
                .unwrap()["value"]
                .as_array()
                .unwrap();

            for loc in locations {
                let location = loc.as_str().unwrap();
                let title = job["title"].as_str().unwrap();
                let link = job["absolute_url"].as_str().unwrap();

                scraped_jobs.push(ScrapedJob {
                    title: title.to_string(),
                    location: location.to_string(),
                    link: link.to_string(),
                });
            }
        }
    }

    // Convert Vector of ScrapedJob into a JobsPayload
    let jobs_payload = JobsPayload::from_scraped_jobs(scraped_jobs, "Cloudflare", data);

    // Return JobsPayload
    Ok(jobs_payload)
}
