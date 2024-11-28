use std::error::Error;

use headless_chrome::{Browser, LaunchOptions};

use crate::models::{
    data::Data,
    scraper::{JobsPayload, ScrapedJob},
};

pub async fn scrape_blizzard(data: &mut Data) -> Result<JobsPayload, Box<dyn Error>> {
    let launch_options = LaunchOptions {
        headless: true,
        window_size: Some((1920, 1080)),
        enable_logging: true,

        ..LaunchOptions::default()
    };
    let browser = Browser::new(launch_options)?;

    let tab = browser.new_tab()?;

    tab.navigate_to("https://careers.blizzard.com/global/en/search-results?from=0&s=1&rk=l-engineering-technology")?;
    tab.wait_for_element("body")?;
    tab.wait_for_element(".results-state.container")?;

    let js = r#"
    const jobsList = document.querySelector('ul[data-ph-id="ph-page-element-page14-X5QcnJ"]');

const jobs = [...jobsList.querySelectorAll("li.jobs-list-item")].map(jobNode => {

    const title = jobNode.querySelector("span").innerText;
    const link = jobNode.querySelector("a").href;
    const locationNode = jobNode.querySelector("span.job-location");
    let location;
        
  if(locationNode) {
      location = locationNode.innerText.split("\n")[1].trim();
      
  } else {
      const locations = [...jobNode.querySelectorAll("li.au-target.location")];

      location = locations.map(loc => loc.innerText.trim()).join(" | ");
  }   
        
        
    return {
        title,
        link,
        location
    }
})

JSON.stringify(jobs)

    "#;

    let mut results = 0;
    let mut scraped_jobs: Vec<ScrapedJob> = Vec::new();
    loop {
        tab.navigate_to(&format!("https://careers.blizzard.com/global/en/search-results?from={}&s=1&rk=l-engineering-technology", results))?;
        tab.wait_for_element("body")?;
        tab.wait_for_element(".results-state.container")?;
        let remote_object = tab.evaluate(js, false)?;

        if let Some(value) = remote_object.value {
            let scraped_jobs_subset: Vec<ScrapedJob> = serde_json::from_str(value.as_str().unwrap())?;
            scraped_jobs.extend(scraped_jobs_subset);
        } else {
            break;
        }

        results += 10;
    }

    let jobs_payload = JobsPayload::from_scraped_jobs(scraped_jobs, &data.data["Blizzard"]);
    data.data.get_mut("Blizzard").unwrap().jobs = jobs_payload.all_jobs.clone();
    data.save();

    Ok(jobs_payload)
}
