use std::error::Error;

use serde_json::Value;

use crate::models::{
    data::Data,
    scraper::{JobsPayload, ScrapedJob},
};

pub async fn scrape_paypal(data: &mut Data) -> Result<JobsPayload, Box<dyn Error>> {
    let mut start = 0;
    let mut scraped_jobs: Vec<ScrapedJob> = Vec::new();
    loop {
        let json: Value = reqwest::get(&format!("https://paypal.eightfold.ai/api/apply/v2/jobs?domain=paypal.com&start={}&num=10&exclude_pid=274904231921&pid=274904231921&Job%20Category=Software%20Development&Job%20Category=Machine%20Learning&Job%20Category=Data%20Science&domain=paypal.com&sort_by=relevance", start)).await?.json().await?;

        let positions = json["positions"].as_array().unwrap();

        if positions.is_empty() {
            break;
        }

        for position in positions {
            for location in position["locations"].as_array().unwrap() {
                let title = position["name"].as_str().unwrap().to_string();
                let link = position["canonicalPositionUrl"]
                    .as_str()
                    .unwrap()
                    .to_string();

                scraped_jobs.push(ScrapedJob {
                    title,
                    link,
                    location: location.as_str().unwrap().to_string(),
                });
            }
        }

        start += 10;
    }

    // Convert Vector of ScrapedJob into a JobsPayload
    let jobs_payload = JobsPayload::from_scraped_jobs(scraped_jobs, "PayPal", data);
    // Return JobsPayload
    Ok(jobs_payload)
}

