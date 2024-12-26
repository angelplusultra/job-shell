use std::error::Error;

use headless_chrome::{Browser, LaunchOptions};
use reqwest::header::{HeaderMap, HeaderValue, USER_AGENT};
use serde_json::Value;

use crate::models::{
    data::Data,
    scraper::{JobsPayload, ScrapedJob},
};

pub async fn scrape_meta(data: &mut Data) -> Result<JobsPayload, Box<dyn Error>> {
    let mut headers = HeaderMap::new();
    headers.insert("accept", HeaderValue::from_static("*/*"));
    headers.insert(
        "accept-language",
        HeaderValue::from_static("en-US,en;q=0.9"),
    );
    headers.insert(
        "content-type",
        HeaderValue::from_static("application/x-www-form-urlencoded"),
    );
    headers.insert("priority", HeaderValue::from_static("u=1, i"));
    headers.insert(
        "sec-ch-ua",
        HeaderValue::from_static(
            "\"Google Chrome\";v=\"131\", \"Chromium\";v=\"131\", \"Not_A Brand\";v=\"24\"",
        ),
    );
    headers.insert("sec-ch-ua-mobile", HeaderValue::from_static("?1"));
    headers.insert(
        "sec-ch-ua-platform",
        HeaderValue::from_static("\"Android\""),
    );
    headers.insert("sec-fetch-dest", HeaderValue::from_static("empty"));
    headers.insert("sec-fetch-mode", HeaderValue::from_static("cors"));
    headers.insert("sec-fetch-site", HeaderValue::from_static("same-origin"));
    headers.insert("x-asbd-id", HeaderValue::from_static("129477"));
    headers.insert(
        "x-fb-friendly-name",
        HeaderValue::from_static("CareersJobSearchResultsQuery"),
    );
    headers.insert("x-fb-lsd", HeaderValue::from_static("AVq53ZVpBuM"));
    headers.insert(
        USER_AGENT,
        HeaderValue::from_static(
            "\"Google Chrome\";v=\"131\", \"Chromium\";v=\"131\", \"Not_A Brand\";v=\"24\"",
        ),
    );

    let client = reqwest::Client::new();
    let response = client.post("https://www.metacareers.com/graphql")
        .headers(headers)
        .body(r#"av=0&__user=0&__a=1&__req=2&__hs=20054.BP%3ADEFAULT.2.0..0.0&dpr=3&__ccg=GOOD&__rev=1018509888&__s=2ppi3o%3Abbrg3o%3Aerkk4k&__hsi=7442044011372323185&__dyn=7xeUmwkHg7ebwKBAg5S1Dxu13wqovzEdEc8uxa1twKzobo1nEhwem0nCq1ewcG0RU2Cwooa81VohwnU14E9k2C0sy0H82NxCawcK1iwmE2ewnE2Lw5XwSyES4E3PwbS1Lwqo3cwbq0x8qw53wtU5K0zU5a&__csr=&lsd=AVq53ZVpBuM&jazoest=2916&__spin_r=1018509888&__spin_b=trunk&__spin_t=1732735896&__jssesw=1&fb_api_caller_class=RelayModern&fb_api_req_friendly_name=CareersJobSearchResultsQuery&variables=%7B%22search_input%22%3A%7B%22q%22%3Anull%2C%22divisions%22%3A%5B%5D%2C%22offices%22%3A%5B%5D%2C%22roles%22%3A%5B%5D%2C%22leadership_levels%22%3A%5B%5D%2C%22saved_jobs%22%3A%5B%5D%2C%22saved_searches%22%3A%5B%5D%2C%22sub_teams%22%3A%5B%5D%2C%22teams%22%3A%5B%22Software%20Engineering%22%2C%22Artificial%20Intelligence%22%5D%2C%22is_leadership%22%3Afalse%2C%22is_remote_only%22%3Afalse%2C%22sort_by_new%22%3Afalse%2C%22results_per_page%22%3Anull%7D%7D&server_timestamps=true&doc_id=9114524511922157"#)
        .send()
        .await?;

    let json = response.json::<Value>().await?;

 let scraped_jobs: Vec<ScrapedJob> = json["data"]["job_search"]
        .as_array()
        .unwrap()
        .iter()
        .flat_map(|v| {
            let title = v["title"].as_str().unwrap().trim().to_string();
            let link = format!(
                "https://www.metacareers.com/jobs/{}",
                v["id"].as_str().unwrap().trim()
            );

            // Create a ScrapedJob for each location
            v["locations"]
                .as_array()
                .unwrap()
                .iter()
                .map(move |l| ScrapedJob {
                    title: title.clone(),
                    location: l.as_str().unwrap().trim().to_string(),
                    link: link.clone(),
                })
        })
        .collect();

    let jobs_payload = JobsPayload::from_scraped_jobs(scraped_jobs, &data.data["Meta"]);

    data.data.get_mut("Meta").unwrap().jobs = jobs_payload.all_jobs.clone();

    data.save();

    Ok(jobs_payload)
}

