use std::error::Error;

use reqwest::Client;
use serde_json::Value;

use crate::models::{
    data::Data,
    scraper::{JobsPayload, ScrapedJob},
};

pub async fn scrape_experian(data: &mut Data) -> Result<JobsPayload, Box<dyn Error>> {
    let mut offset = 0;
    let mut scraped_jobs: Vec<ScrapedJob> = Vec::new();

    loop {
        let url = format!(
            "https://api.smartrecruiters.com/v1/companies/experian/postings?offset={}",
            offset
        );
        let json: Value = Client::new().get(&url).send().await?.json().await?;

        let mut content = json["content"].as_array().unwrap().clone();

        if content.is_empty() {
            break;
        }

        content.retain(|v| v["department"]["id"].as_str().unwrap() == "2618908");

        let scraped_jobs_subset: Vec<ScrapedJob> = content
            .iter()
            .map(|v| {
                let title = v["name"].as_str().unwrap().trim().to_string();
                let location = format!(
                    "{}, {} {}",
                    v["location"]["city"].as_str().unwrap_or("N/A").trim(),
                    v["location"]["region"]
                        .as_str()
                        .unwrap_or("N/A")
                        .trim()
                        .to_uppercase(),
                    v["location"]["country"]
                        .as_str()
                        .unwrap_or("N/A")
                        .trim()
                        .to_uppercase()
                );

                let link = format!(
                    "https://jobs.smartrecruiters.com/Experian/{}",
                    v["id"].as_str().unwrap()
                );

                ScrapedJob {
                    title,
                    location,
                    link,
                }
            })
            .collect();

        scraped_jobs.extend(scraped_jobs_subset);
        offset += 100;
    }
    // Convert Vector of ScrapedJob into a JobsPayload
    let jobs_payload = JobsPayload::from_scraped_jobs(scraped_jobs, "Experian", data);

    // Return JobsPayload
    Ok(jobs_payload)
}
