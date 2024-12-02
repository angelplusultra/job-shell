use std::error::Error;

use reqwest::Client;
use serde_json::Value;

use crate::models::{
    data::Data,
    scraper::{JobsPayload, ScrapedJob},
};

pub async fn scrape_chase(data: &mut Data) -> Result<JobsPayload, Box<dyn Error>> {
    let mut offset = 0;
    let mut scraped_jobs: Vec<ScrapedJob> = Vec::new();

    loop {
        let mut url  = format!("https://jpmc.fa.oraclecloud.com/hcmRestApi/resources/latest/recruitingCEJobRequisitions?onlyData=true&expand=requisitionList.secondaryLocations,flexFieldsFacet.values,requisitionList.requisitionFlexFields&finder=findReqs;siteNumber=CX_1002,facetsList=LOCATIONS%3BWORK_LOCATIONS%3BWORKPLACE_TYPES%3BTITLES%3BCATEGORIES%3BORGANIZATIONS%3BPOSTING_DATES%3BFLEX_FIELDS,limit=200,lastSelectedFacet=CATEGORIES,selectedCategoriesFacet=300000086152753,sortBy=POSTING_DATES_DESC,offset={}", offset);

        let json = Client::new().get(url).send().await?.json::<Value>().await?;

        let jobs = json["items"][0]["requisitionList"].as_array().unwrap();

        if jobs.is_empty() {
            break;
        }

        let scraped_jobs_subset: Vec<ScrapedJob> = jobs.iter().map(|v| ScrapedJob {
            title: v["Title"].as_str().unwrap().trim().to_string(),
            location: v["PrimaryLocation"].as_str().unwrap().trim().split(",").take(2).collect::<Vec<&str>>().join(","),
            link: format!("https://jpmc.fa.oraclecloud.com/hcmUI/CandidateExperience/en/sites/CX_1002/job/{}", v["Id"].as_str().unwrap())

        }).collect();

        scraped_jobs.extend(scraped_jobs_subset);
        offset += 200;
    }
    // Convert Vector of ScrapedJob into a JobsPayload
    let jobs_payload = JobsPayload::from_scraped_jobs(scraped_jobs, &data.data["Chase"]);

    // REMEBER TO SAVE THE NEW JOBS TO THE DATA STATE
    data.data.get_mut("Chase").unwrap().jobs = jobs_payload.all_jobs.clone();
    data.save();

    // Return JobsPayload
    Ok(jobs_payload)
}

