use std::error::Error;

use serde_json::Value;

use crate::models::{
    data::Data,
    scraper::{JobsPayload, ScrapedJob},
};

pub async fn scrape_nike(data: &mut Data) -> Result<JobsPayload, Box<dyn Error>> {
    let mut offset = 0;
    let mut scraped_jobs: Vec<ScrapedJob> = Vec::new();
    loop {
        let url = format!("https://jobs.nike.com/cms/api/v1/nike/search/jobs/?offset={}&limit=100&sort_key=posting_start_date&lang=en&categories=Technology&sort_dir=DESC", offset);
        let mut json: Value = reqwest::get(&url).await?.json().await?;

        let jobs = json["jobs"].as_array_mut().unwrap();

        if jobs.len() == 0 {
            break;
        }

        // INFO: Filtering out jobs that are not software engineering roles
        let valid_job_titles = [
            "tech lead",
            "software engineer",
            "backend developer",
            "frontend developer",
            "full stack developer",
            "software developer",
            "engineering manager",
            "engineering director",
            "engineering lead",
            "web developer",
            "mobile developer",
        ];

        jobs.retain(|job| {
            let title = job["atsPayload"]["content"]["title"]
                .as_str()
                .unwrap()
                .to_lowercase();
            valid_job_titles
                .iter()
                .any(|valid_title| title.contains(valid_title))
        });
        for job in jobs {
            let title = job["atsPayload"]["content"]["title"]
                .as_str()
                .unwrap()
                .to_string();
            let link = job["postUrl"].as_str().unwrap().to_string();

            let location = format!(
                "{}, {}, {}",
                job["atsPayload"]["location"]["administrational"]["city"]
                    .as_str()
                    .unwrap(),
                job["atsPayload"]["location"]["administrational"]["stateProvince"]
                    .as_str()
                    .unwrap(),
                job["atsPayload"]["location"]["administrational"]["country"]
                    .as_str()
                    .unwrap()
            );

            scraped_jobs.push(ScrapedJob {
                title,
                location,
                link,
            });
        }
        offset += 100;
    }

    // Convert Vector of ScrapedJob into a JobsPayload
    let jobs_payload = JobsPayload::from_scraped_jobs(scraped_jobs, "Nike", data);

    // Return JobsPayload
    Ok(jobs_payload)
}

