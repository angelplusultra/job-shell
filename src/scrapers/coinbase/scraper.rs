use std::error::Error;

use headless_chrome::{Browser, LaunchOptions};

use crate::models::{
    data::Data,
    scraper::{JobsPayload, ScrapedJob},
};

pub async fn scrape_coinbase(data: &mut Data) -> Result<JobsPayload, Box<dyn Error>> {
    let launch_options = LaunchOptions {
        headless: false,
        window_size: Some((1920, 1080)),
        enable_logging: false,

        ..LaunchOptions::default()
    };
    let browser = Browser::new(launch_options)?;

    let tab = browser.new_tab()?;

    tab.navigate_to("https://www.coinbase.com/careers/positions")?;
    tab.wait_for_element("body")?;
    // tab.wait_for_element(".Positions__PositionsColumn-sc-48777b23-7.eQUcAP")?;

    let elements = tab.find_elements(".Department__Wrapper-sc-3686241a-0.dACtTU")?;

    for element in elements {
        let p = element.find_element("p")?;

        let department_title = p.get_inner_text()?;

        if department_title.contains("Engineering") {
            p.click()?;
            println!("{} clicked", department_title);
        }
    }

    let scraped_jobs = tab.evaluate(
        r##"
const engDeps = [...document.querySelectorAll(".Department__Wrapper-sc-3686241a-0.dACtTU")]
    .filter(dep => dep.querySelector("p").textContent.includes("Engineering"));

// engDeps.forEach(dep => dep.querySelector("p").click());

const jobs = engDeps.flatMap(dep =>
    [...dep.querySelectorAll('div[class^="Department__Job-sc"]')].map(job => ({
        title: job.querySelector("a").textContent,
        link: job.querySelector("a").href,
        location: job.querySelector("p").textContent
    }))
);

JSON.stringify(jobs);

    "##,
        false,
    )?;

    let scraped_jobs: Vec<ScrapedJob> =
        serde_json::from_str(scraped_jobs.value.unwrap().as_str().unwrap()).unwrap();

    let jobs_payload = JobsPayload::from_scraped_jobs(scraped_jobs, "Coinbase", data);

    Ok(jobs_payload)
}
