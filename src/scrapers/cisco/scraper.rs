use core::panic;
use std::error::Error;

use reqwest::Client;
use scraper::{Html, Selector};

use crate::{error::AppResult, models::{
    data::Data,
    scraper::{JobsPayload, ScrapedJob},
}};

pub async fn scrape_cisco(data: &mut Data) -> AppResult<JobsPayload> {
    // Fetch the html

    //         let html_string = Client::new().get("https://jobs.cisco.com/jobs/SearchJobs/?21181=%5B187%5D&21181_format=6023&listFilterMode=1&projectOffset=300").send().await?.text().await?;
    //
    //
    // println!("{html_string}");

    let mut offset = 0;
    let mut scraped_jobs: Vec<ScrapedJob> = Vec::new();
    loop {
        let url = format!("https://jobs.cisco.com/jobs/SearchJobs/?21181=%5B187%5D&21181_format=6023&listFilterMode=1&projectOffset={offset}");
        let html_string = Client::new().get(&url).send().await?.text().await?;

        let document = Html::parse_document(&html_string);

        let selector = Selector::parse("tr").unwrap();
        let table_rows = document.select(&selector);

        // Check for "No results" message
        let no_results = document
            .select(&Selector::parse("td[colspan='5']").unwrap())
            .any(|el| el.text().collect::<String>().contains("No results"));

        if no_results {
            break;
        }

        for table_row in table_rows {
            // Get the first td which contains the job title and link
            if let Some(title_cell) = table_row
                .select(&Selector::parse("td:first-child").unwrap())
                .next()
            {
                // Extract the link and title from the anchor tag
                if let Some(anchor) = title_cell.select(&Selector::parse("a").unwrap()).next() {
                    let title = anchor.text().collect::<String>().trim().to_string();
                    let link = anchor.value().attr("href").unwrap_or_default().to_string();

                    // Get the location from the fourth td
                    let location = table_row
                        .select(&Selector::parse("td:nth-child(4)").unwrap())
                        .next()
                        .map(|cell| cell.text().collect::<String>().trim().to_string())
                        .unwrap_or_default();

                    let job = ScrapedJob {
                        title,
                        location,
                        link,
                    };

                    scraped_jobs.push(job);
                }
            }
        }

        offset += 25;
    }

    // Convert Vector of ScrapedJob into a JobsPayload
    let jobs_payload = JobsPayload::from_scraped_jobs(scraped_jobs, "Cisco", data);

    // Return JobsPayload
    Ok(jobs_payload)
}

#[cfg(test)]
mod test {
    use super::*;

    #[tokio::test]
    async fn test_scrape_cisco() {
        let mut data = Data::default();

        let v = scrape_cisco(&mut data).await.unwrap();
    }
}
