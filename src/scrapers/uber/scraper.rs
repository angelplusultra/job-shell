use std::{error::Error, thread::sleep, time::Duration};

use reqwest::Client;
use serde_json::{json, Value};

use crate::{error::AppResult, models::{
    data::Data,
    scraper::{JobsPayload, ScrapedJob},
}};

pub async fn scrape_uber(data: &mut Data) -> AppResult<JobsPayload> {
    let mut page = 0;

    let mut scraped_jobs: Vec<ScrapedJob> = Vec::new();
    loop {
        let body = json!({
            "limit": 100,
            "page": page,
            "params": {
            "department": ["Engineering"],
        }
        });

        let url = "https://www.uber.com/api/loadSearchJobsResults?localeCode=en";

        let json: Value = Client::new()
            .post(url)
            .body(body.to_string())
            .header("Content-Type", "application/json")
            .header("x-csrf-token", "x")
            .send()
            .await?
            .json()
            .await?;

        if let Some(jobs) = json["data"]["results"].as_array() {
            for job in jobs {
                let title = job["title"].as_str().unwrap();
                let id = job["id"].as_number().unwrap().as_i64().unwrap();

                for loc in job["allLocations"].as_array().unwrap() {
                    let link = format!("https://www.uber.com/global/en/careers/list/{}/", id);
                    // cities are optional
                    let city = loc["city"].as_str();

                    let country = loc["countryName"].as_str().unwrap();

                    let location = match city {
                        Some(city) => format!("{}, {}", city, country),
                        None => country.to_string(),
                    };
                    let scraped_job = ScrapedJob {
                        title: title.to_string(),
                        location,
                        link,
                    };

                    scraped_jobs.push(scraped_job);
                }
            }
        } else {
            break;
        }

        page += 1;
    }

    let jobs_payload = JobsPayload::from_scraped_jobs(scraped_jobs, "Uber", data);

    Ok(jobs_payload)
}
