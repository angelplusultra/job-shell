use std::{error::Error, thread::sleep, time::Duration};

use headless_chrome::{Browser, LaunchOptions};
use reqwest::Client;
use serde_json::{json, Value};

use crate::models::{
    data::Data,
    gemini::Root,
    scraper::{Job, JobsPayload},
};

pub async fn scrape_square(data: &mut Data) -> Result<JobsPayload, Box<dyn Error>> {
    let options = LaunchOptions {
        headless: false,
        window_size: Some((1920, 1080)),
        enable_logging: true,

        ..LaunchOptions::default()
    };

    let browser = Browser::new(options)?;

    let tab = browser.new_tab()?;

    tab.navigate_to("https://block.xyz/careers/jobs?businessUnits%5B%5D=square&teams%5B%5D=Software%20Engineering")?;

    tab.wait_for_element("body")?;
    let button = tab.wait_for_element(r#"button[role="button"]"#)?;
    println!("Button loaded");
    //
    button.click()?;
    println!("Button clicked");

    // tab.wait_for_element(".job-function-section")?;
    //     tab.evaluate(
    //         r#"document.querySelector('button[role="button"]').click()
    // "#,
    //         false,
    //     );
    //
    //     let v = button.call_js_fn("this.click()", vec![], false);
    // tab.evaluate(r#"window.scrollTo(0, document.body.scrollHeight)"#, false)?;

    let expression = r#"
function loadAllJobsAndExtract() {
  // Scroll to the bottom of the page
  window.scrollTo(0, document.body.scrollHeight);

  // Function to click the button until it no longer exists
  async function clickLoadMore() {
    const button = document.querySelector('button.css-125xjb8:nth-child(1)');
    if (button) {
      button.click();
      // Recursively click the button
      clickLoadMore();
    }
  }

  clickLoadMore();

  // Extract inner HTML content of each child element in .jobs-stack
  const jobElements = document.querySelectorAll('.jobs-stack > *');
  const jobContents = Array.from(jobElements).map(el => el.innerHTML);

  return jobContents;
}

// Usage
loadAllJobsAndExtract()
    "#;

    // let v = tab.evaluate(expression, false)?;

    // BUG: What the actual fuck why is it scraping the website so slowly.
    while let Ok(load_more_sec) = tab.find_element(".load-more") {
        if let Ok(button) = load_more_sec.find_element("button") {
            button.click()?;
            println!("Button clicked, loading more jobs");
        } else {
            // Exit if no button is found
            break;
        }
    }

    let remote_object = tab.evaluate(
        r#"document.querySelector(".job-function-section").innerHTML"#,
        false,
    )?;

    let html = remote_object.value.unwrap().to_string();

    let client = Client::new();

    let prompt = format!(
        r#"
    Prompt: Please parse the jobs in this HTML and return a JSON string in the following format:

    "{{
        "jobs": [{{
        "title": "Software Engineer III",
        "location": "Los Angeles, CA (Remote)",
        "link": "https://thehreftoapply.com"

        }}]
    }}"


    IMPORTANT: Do not format the JSON in a markdown style with backticks, for example do not do this: 

    ```json

    ```


    Just return raw string JSON.



    HTML: {}
    "#,
        html
    );
    todo!("Need to fix api key");
    let response = client
        .post("https://generativelanguage.googleapis.com/v1beta/models/gemini-1.5-flash:generateContent?key=")
        .json(&json!({
            "contents": {
            "parts": [{"text": prompt}]
        }
        }))
        .send()
        .await?
        .json::<Root>()
        .await?;

    println!("{:#?}", response.candidates);

    let job_data: Value = serde_json::from_str(&response.candidates[0].content.parts[0].text)?;

    let jobs: Vec<Job> = serde_json::from_value(job_data["jobs"].clone())?;

    println!("{:#?}", &jobs);

    let square_jobs_payload = JobsPayload::from_jobs(&jobs, &data.square.jobs);

    Ok(square_jobs_payload)
}
