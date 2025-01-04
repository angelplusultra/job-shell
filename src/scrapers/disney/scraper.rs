use std::error::Error;

use headless_chrome::{Browser, LaunchOptions};

use crate::{error::AppResult, models::{
    data::Data,
    scraper::{JobsPayload, ScrapedJob},
}};

pub async fn scrape_disney(data: &mut Data) -> AppResult<JobsPayload> {
    let launch_options = LaunchOptions {
        headless: true,
        window_size: Some((1920, 1080)),
        enable_logging: true,

        ..LaunchOptions::default()
    };
    let browser = Browser::new(launch_options)?;

    let tab = browser.new_tab()?;

    tab.navigate_to("https://www.disneycareers.com/en/search-jobs?acm=26715,74122,74124,8221776&alrpm=ALL&ascf=[%7B%22key%22:%22ALL%22,%22value%22:%22%22%7D]")?;
    tab.wait_for_element("body")?;
    tab.wait_for_element("body")?;

    let mut next_button_result = tab.wait_for_element("a.next");

    let mut total_scraped_jobs: Vec<ScrapedJob> = Vec::new();
    loop {
        // Check for the next button
        let next_button_result = tab.wait_for_element("a.next");
        let next_button = match next_button_result {
            Ok(button) => button,
            Err(_) => break, // Exit the loop if the button is not found
        };

        // Check if the button is disabled
        if let Some(classes) = next_button.get_attribute_value("class")? {
            if classes.contains("disabled") {
                break; // Exit the loop if the button is disabled
            }
        } else {
            break; // Exit the loop if the class attribute is missing
        }

        // Scrape jobs or perform actions
        tab.wait_for_element("body")?;
        let remote_object = tab.evaluate(
            r#"


    

JSON.stringify([...document.querySelector('#search-results-list')
        .querySelectorAll('ul')[1]
        .querySelectorAll('li')]
    .map(item => {
let statesOfAmerica = [
  "Alabama",
  "Alaska",
  "Arizona",
  "Arkansas",
  "California",
  "Colorado",
  "Connecticut",
  "Delaware",
  "Florida",
  "Georgia",
  "Hawaii",
  "Idaho",
  "Illinois",
  "Indiana",
  "Iowa",
  "Kansas",
  "Kentucky",
  "Louisiana",
  "Maine",
  "Maryland",
  "Massachusetts",
  "Michigan",
  "Minnesota",
  "Mississippi",
  "Missouri",
  "Montana",
  "Nebraska",
  "Nevada",
  "New Hampshire",
  "New Jersey",
  "New Mexico",
  "New York",
  "North Carolina",
  "North Dakota",
  "Ohio",
  "Oklahoma",
  "Oregon",
  "Pennsylvania",
  "Rhode Island",
  "South Carolina",
  "South Dakota",
  "Tennessee",
  "Texas",
  "Utah",
  "Vermont",
  "Virginia",
  "Washington",
  "West Virginia",
  "Wisconsin",
  "Wyoming"
];

let location = item.querySelector("span.job-location").textContent.trim();

let locations = location.split("/");

const states = locations.map(loc => {
    const v = loc.split(",")

    return v[1] ? v[1].trim() : v[0].trim()
})

for(const state of states) {
    if(statesOfAmerica.includes(state)) {
        location = state
        break;
    }
}
        

return ({
        title: item.querySelector("h2").textContent.trim(),
        location,
        link: item.querySelector("a").href.trim()
    })}))










//     JSON.stringify(
//     [...document.querySelector('#search-results-list')
//         .querySelectorAll('ul')[1]
//         .querySelectorAll('li')]
//     .map(item => ({
//         title: item.querySelector("h2").textContent.trim(),
//         location: item.querySelector("span.job-location").textContent.trim(),
//         link: item.querySelector("a").href.trim()
//     }))
// );
    "#,
            false,
        )?;

        let scraped_jobs: Vec<ScrapedJob> =
            serde_json::from_str(remote_object.value.unwrap().as_str().unwrap()).unwrap();

        total_scraped_jobs.extend(scraped_jobs);

        tab.evaluate(r#"document.querySelector("a.next").click()"#, false)?;
    }

    let jobs_payload = JobsPayload::from_scraped_jobs(total_scraped_jobs, "Disney", data);

    Ok(jobs_payload)
}
