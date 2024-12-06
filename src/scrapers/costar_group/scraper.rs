use std::error::Error;

use reqwest::Client;
use serde_json::Value;

use crate::models::{
    data::Data,
    scraper::{JobsPayload, ScrapedJob},
};

pub async fn scrape_costar_group(data: &mut Data) -> Result<JobsPayload, Box<dyn Error>> {
    let mut start = 0;
    let mut scraped_jobs: Vec<ScrapedJob> = Vec::new();

    loop {
        let url = format!("https://careers.costargroup.com/api/apply/v2/jobs?domain=costar.com&start={start}&num=10&exclude_pid=446702351152&pid=446702351152&business_unit=Software%20Engineering&domain=costar.com&sort_by=relevance");
        let data: Value = Client::new().get(&url).send().await?.json().await?;

        let positions = data["positions"].as_array().unwrap();

        if positions.is_empty() {
            break;
        }

        let scraped_jobs_subset: Vec<ScrapedJob> = positions
            .iter()
            .map(|v| {
                let title = v["name"].as_str().unwrap().trim().to_string();
                let location = v["location"].as_str().unwrap().trim().to_string();
                let link = v["canonicalPositionUrl"]
                    .as_str()
                    .unwrap()
                    .trim()
                    .to_string();

                ScrapedJob {
                    title,
                    link,
                    location,
                }
            })
            .collect();

        scraped_jobs.extend(scraped_jobs_subset);

        start += 10;
    }

    // Convert Vector of ScrapedJob into a JobsPayload
    let jobs_payload = JobsPayload::from_scraped_jobs(scraped_jobs, &data.data["CoStar Group"]);

    // REMEBER TO SAVE THE NEW JOBS TO THE DATA STATE
    data.data.get_mut("CoStar Group").unwrap().jobs = jobs_payload.all_jobs.clone();
    data.save();

    // Return JobsPayload
    Ok(jobs_payload)
}

