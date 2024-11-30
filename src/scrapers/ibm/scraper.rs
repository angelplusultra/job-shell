use std::error::Error;

use reqwest::{
    header::{HeaderMap, HeaderValue, ACCEPT, ACCEPT_LANGUAGE, CONTENT_TYPE, REFERER},
    Client, ClientBuilder,
};
use serde_json::{json, Value};

use crate::models::{
    data::Data,
    scraper::{JobsPayload, ScrapedJob},
};

pub async fn scrape_ibm(data: &mut Data) -> Result<JobsPayload, Box<dyn Error>> {
    let mut from = 0;

    let mut p = 1;
    let mut scraped_jobs: Vec<ScrapedJob> = Vec::new();
    let mut headers = HeaderMap::new();

    // Add headers
    headers.insert(
        ACCEPT,
        HeaderValue::from_static("application/json, text/plain, */*"),
    );
    headers.insert(ACCEPT_LANGUAGE, HeaderValue::from_static("en-US,en;q=0.9"));
    headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
    headers.insert(
        "sec-ch-ua",
        HeaderValue::from_static("\"Chromium\";v=\"131\", \"Not_A Brand\";v=\"24\""),
    );
    headers.insert("sec-ch-ua-mobile", HeaderValue::from_static("?0"));
    headers.insert("sec-ch-ua-platform", HeaderValue::from_static("\"macOS\""));
    headers.insert("sec-fetch-dest", HeaderValue::from_static("empty"));
    headers.insert("sec-fetch-mode", HeaderValue::from_static("cors"));
    headers.insert("sec-fetch-site", HeaderValue::from_static("same-site"));
    headers.insert(REFERER, HeaderValue::from_static("https://www.ibm.com/"));
    headers.insert(
        "Referrer-Policy",
        HeaderValue::from_static("strict-origin-when-cross-origin"),
    );

    let client = ClientBuilder::new().default_headers(headers).build()?;

    loop {
        let body = json!({"appId":"careers", "from": from, "p": p, "scopes":["careers"],"query":{"bool":{"must":[]}},"post_filter":{"bool":{"must":[{"term":{"field_keyword_08":"Software Engineering"}},{"term":{"field_keyword_05":"United States"}}]}},"aggs":{"field_keyword_172":{"filter":{"bool":{"must":[{"term":{"field_keyword_08":"Software Engineering"}},{"term":{"field_keyword_05":"United States"}}]}},"aggs":{"field_keyword_17":{"terms":{"field":"field_keyword_17","size":6}},"field_keyword_17_count":{"cardinality":{"field":"field_keyword_17"}}}},"field_keyword_083":{"filter":{"term":{"field_keyword_05":"United States"}},"aggs":{"field_keyword_08":{"terms":{"field":"field_keyword_08","size":6}},"field_keyword_08_count":{"cardinality":{"field":"field_keyword_08"}}}},"field_keyword_184":{"filter":{"bool":{"must":[{"term":{"field_keyword_08":"Software Engineering"}},{"term":{"field_keyword_05":"United States"}}]}},"aggs":{"field_keyword_18":{"terms":{"field":"field_keyword_18","size":6}},"field_keyword_18_count":{"cardinality":{"field":"field_keyword_18"}}}},"field_keyword_055":{"filter":{"term":{"field_keyword_08":"Software Engineering"}},"aggs":{"field_keyword_05":{"terms":{"field":"field_keyword_05","size":1000}},"field_keyword_05_count":{"cardinality":{"field":"field_keyword_05"}}}}},"size":30,"sort":[{"_score":"desc"},{"pageviews":"desc"}],"lang":"zz","localeSelector":{},"sm":{"query":"","lang":"zz"},"_source":["_id","title","url","description","language","entitled","field_keyword_17","field_keyword_08","field_keyword_18","field_keyword_19"]});

        let json: Value = client
            .post("https://www-api.ibm.com/search/api/v2")
            .body(serde_json::to_string(&body).unwrap())
            .send()
            .await?
            .json()
            .await?;

        let hits = json["hits"]["hits"].as_array().unwrap();
        if hits.is_empty() {
            break;
        }

        let scraped_jobs_subset: Vec<ScrapedJob> = hits
            .iter()
            .map(|v| {
                let title = format!(
                    "{} {}",
                    v["_source"]["title"].as_str().unwrap().trim().to_string(),
                    v["_source"]["field_keyword_18"].as_str().unwrap().trim()
                );

                let location = v["_source"]["field_keyword_19"]
                    .as_str()
                    .unwrap()
                    .trim()
                    .to_string();
                let link = v["_source"]["url"].as_str().unwrap().trim().to_string();

                ScrapedJob {
                    title,
                    location,
                    link,
                }
            })
            .collect();

        scraped_jobs.extend(scraped_jobs_subset);

        from += 30;
        p += 1;
    }

    // Convert Vector of ScrapedJob into a JobsPayload
    let jobs_payload = JobsPayload::from_scraped_jobs(scraped_jobs, &data.data["IBM"]);

    // REMEBER TO SAVE THE NEW JOBS TO THE DATA STATE
    data.data.get_mut("IBM").unwrap().jobs = jobs_payload.all_jobs.clone();
    data.save();

    // Return JobsPayload
    Ok(jobs_payload)
}

