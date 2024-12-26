use std::error::Error;

use headless_chrome::{Browser, LaunchOptions};

use crate::models::{
    data::Data,
    scraper::{JobsPayload, ScrapedJob},
};

pub async fn scrape_stripe(data: &mut Data) -> Result<JobsPayload, Box<dyn Error>> {
    let launch_options = LaunchOptions {
        headless: true,
        window_size: Some((1920, 1080)),
        enable_logging: true,

        ..LaunchOptions::default()
    };
    let browser = Browser::new(launch_options)?;

    let tab = browser.new_tab()?;
    let mut skip_count = 0;
    let mut scraped_jobs: Vec<ScrapedJob> = Vec::new();

    loop {
        let url = format!(
        "https://stripe.com/jobs/search?teams=Banking+as+a+Service&teams=Climate&teams=Connect&teams=Crypto&teams=Mobile&teams=Money+Movement+and+Storage&teams=New+Financial+Products&teams=Payments&teams=Platform&teams=Professional+Services&teams=Revenue+%26+Financial+Automation&teams=Stripe+Tax&teams=Terminal&skip={}",
        skip_count
    );
        tab.navigate_to(&url)?;
        tab.wait_for_element("body")?;
        let a_tags = tab.wait_for_elements("a.Link.JobsPagination__link")?;

        let next_button_opt = a_tags
            .iter()
            .find(|el| el.get_inner_text().unwrap_or_default() == "Next");

        let remote_object = tab.evaluate(r#"
        
const tableRows = [...document.querySelectorAll(".TableRow")];
const jobs = []
for(const row of tableRows) {
    const title = row.querySelector(".Link.JobsListings__link")?.textContent;

    if(!title) continue;
    
    const location = row.querySelector(".TableCell.JobsListings__tableCell.JobsListings__tableCell--country")?.textContent.trim();
    const country = row.querySelector("img")?.alt;

    const isRemote = location.includes("Remote");

const locationString = `${isRemote ? "Remote" : location}, ${country}`;
   

    jobs.push({
        title,
        location: locationString,
        link: row.querySelector("a").href
        
        
    })

    
}



JSON.stringify(jobs);
        "#, false)?;

        if let Some(scraped_jobs_subset) = remote_object.value {
            let scraped_jobs_subset: Vec<ScrapedJob> =
                serde_json::from_str(scraped_jobs_subset.as_str().unwrap()).unwrap();
            scraped_jobs.extend(scraped_jobs_subset);
        } else {
            break;
        }

        if next_button_opt.is_none() {
            break;
        }
        // Click the next button and continue
        let next_button = next_button_opt.unwrap();
        next_button.click()?;

        skip_count += 100;
    }

    // Acquire Vector of ScrapedJob
    // Convert Vector of ScrapedJob into a JobsPayload
    let jobs_payload = JobsPayload::from_scraped_jobs(scraped_jobs, "Stripe", data);

    // Return JobsPayload
    Ok(jobs_payload)
}
