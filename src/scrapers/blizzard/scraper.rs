use std::error::Error;

use headless_chrome::{Browser, LaunchOptions};

use crate::models::{
    data::Data,
    scraper::{JobsPayload, ScrapedJob},
};

pub async fn scrape_blizzard(data: &mut Data) -> Result<JobsPayload, Box<dyn Error>> {
    let launch_options = LaunchOptions {
        headless: false,
        window_size: Some((1920, 1080)),
        enable_logging: true,

        ..LaunchOptions::default()
    };
    let browser = Browser::new(launch_options)?;

    let tab = browser.new_tab()?;

    tab.navigate_to("https://careers.blizzard.com/global/en/search-results?rk=l-engineering-technology&sortBy=Most%20relevant")?;
    tab.wait_for_element("body")?;
    tab.wait_for_element(".results-state.container")?;

    //     let scraped_jobs = tab.evaluate(
    //         r##"
    //
    // // DELETE AND REPLACE WITH CUSTOM JS LOGIC
    // const engJobs = document.querySelector("#jobs-16253")
    //
    // const jobsPayload = Array.from(engJobs.querySelectorAll(".job")).map(j => {
    //     const title = j.querySelector(".job-title").innerHTML;
    //     const location = j.querySelector(".job-location").innerHTML;
    //     const link = j.querySelector("a").href;
    //
    //     return {
    //         title,
    //         location,
    //         link
    //     }
    // })
    //
    // JSON.stringify(jobsPayload);
    //     "##,
    //         false,
    //     )?;

    let mut button_result = tab.wait_for_element("#acc-skip-content > div.body-wrapper.ph-page-container > div > div > div > div.col-lg-8.col-md-8.col-sm-7 > section:nth-child(2) > div > div > div > div.pagination-block.au-target > ul > li:nth-child(5) > a");

    while let Ok(button) = button_result {
        button.click()?;
        tab.evaluate("window.scrollTo(0, document.body.scrollHeight);", false)?;
        tab.wait_for_element("#jobs-list-item")?;
        button_result = tab.find_element("#acc-skip-content > div.body-wrapper.ph-page-container > div > div > div > div.col-lg-8.col-md-8.col-sm-7 > section:nth-child(2) > div > div > div > div.pagination-block.au-target > ul > li:nth-child(5) > a");
    }

    todo!();
    // let scraped_jobs: Vec<ScrapedJob> =
    //     serde_json::from_str(scraped_jobs.value.unwrap().as_str().unwrap()).unwrap();
    //
    // let jobs_payload = JobsPayload::from_scraped_jobs(scraped_jobs, &data.data["Blizzard"]);
    //
    // data.data.get_mut("Blizzard").unwrap().jobs = jobs_payload.all_jobs.clone();
    //
    // data.save();

    // Ok(jobs_payload)
}

