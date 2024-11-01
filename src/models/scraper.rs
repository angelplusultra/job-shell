// use headless_chrome::{Browser, LaunchOptions};
// use serde::{Deserialize, Serialize};
// use std::{collections::HashSet, error::Error};
//
// use crate::{utils::snapshots::write_to_snapshots, Snapshots};
//
// #[derive(Debug, Deserialize, Serialize, Clone)]
// pub struct Job {
//     pub title: String,
//     pub location: String,
//     pub link: String,
// }
//
// #[derive(Debug, Deserialize, Serialize, Clone)]
// pub struct JobsPayload {
//     pub are_new_jobs: bool,
//     pub new_jobs: Vec<Job>,
//     pub all_jobs: Vec<Job>,
// }
//
// struct Scraper<'a, F> {
//     launch_options: LaunchOptions<'a>,
//     url: &'static str,
//     scrape_fn: F,
// }
//
//
// impl <'a,F>Scraper<'a, F> {
//     pub fn scrape(&self) -> Result<JobsPayload, Box<dyn Error>> {
//         (self.scrape_fn)()
//
//     }
// }
//
// pub async fn scrape_weedmaps(snapshots: &mut Snapshots) -> Result<JobsPayload, Box<dyn Error>> {
//     let options = LaunchOptions {
//         headless: false,
//         window_size: Some((1920, 1080)),
//         enable_logging: true,
//
//         ..LaunchOptions::default()
//     };
//
//     let browser = Browser::new(options)?;
//
//     let tab = browser.new_tab()?;
//
//     tab.navigate_to("https://boards.greenhouse.io/embed/job_board?for=weedmaps77&b=https%3A%2F%2Fweedmaps.com%2Fcareers")?;
//
//     // Wait for page to load (wait for body element)
//     tab.wait_for_element("body")?;
//
//     // get weedmaps software jobs
//     let links = tab.evaluate(
//         r#"
// JSON.stringify(Array.from(document.querySelectorAll('div[department_id="4069853002,4069854002"]')).map(job => {
//     const link = job.querySelector("a").href;
//     const [title, location] = job.innerText.split("\n")
//     return {
//         title,
//         location,
//         link
//     }
// }))
//     "#,
//         false,
//     )?;
//
//     let scraped_jobs: Vec<Job> = serde_json::from_str(links.value.unwrap().as_str().unwrap())?;
//
//     let old_jobs: HashSet<&str> = snapshots
//         .weedmaps
//         .iter()
//         .map(|j| j.title.as_str())
//         .collect();
//
//     let new_jobs = scraped_jobs
//         .iter()
//         .filter(|&j| !old_jobs.contains(j.title.as_str()))
//         .cloned()
//         .collect::<Vec<Job>>();
//
//     snapshots.weedmaps = scraped_jobs.clone();
//     write_to_snapshots(&snapshots);
//
//     Ok(JobsPayload {
//         are_new_jobs: new_jobs.len() > 0,
//         new_jobs,
//         all_jobs: scraped_jobs,
//     })
// }
// fn shit() {
//     let weedmaps_scraper = Scraper {
//         url: "weedmaps.com",
//         launch_options: LaunchOptions::default(),
//         scrape_fn: scrape_weedmaps,
//     };
// }
//
//

use crate::utils::snapshots::write_to_snapshots;
use headless_chrome::{Browser, LaunchOptions};
use serde::{Deserialize, Serialize};
use std::future::Future;
use std::pin::Pin;
use std::{collections::HashSet, error::Error};

use super::snapshots::Snapshots;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Job {
    pub title: String,
    pub location: String,
    pub link: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct JobsPayload {
    pub are_new_jobs: bool,
    pub new_jobs: Vec<Job>,
    pub all_jobs: Vec<Job>,
}

impl JobsPayload {
    pub fn from_jobs(jobs: &Vec<Job>, snapshot_of_jobs: &Vec<Job>) -> Self {
        let old_jobs: HashSet<&str> = snapshot_of_jobs.iter().map(|j| j.title.as_str()).collect();

        let new_jobs = jobs
            .iter()
            .filter(|&j| !old_jobs.contains(j.title.as_str()))
            .cloned()
            .collect::<Vec<Job>>();

        JobsPayload {
            are_new_jobs: new_jobs.len() > 0,
            new_jobs,
            all_jobs: jobs.clone(),
        }
    }
}

// Define the Scraper struct with higher-ranked trait bounds
pub struct Scraper<F>
where
    F: for<'a> Fn(
        &'a mut Snapshots,
    )
        -> Pin<Box<dyn Future<Output = Result<JobsPayload, Box<dyn Error>>> + Send + 'a>>,
{
    pub launch_options: LaunchOptions<'static>,
    pub url: &'static str,
    pub scrape_fn: F,
}

impl<F> Scraper<F>
where
    F: for<'a> Fn(
        &'a mut Snapshots,
    )
        -> Pin<Box<dyn Future<Output = Result<JobsPayload, Box<dyn Error>>> + Send + 'a>>,
{
    // Make scrape an async function
    pub async fn scrape(&self, snapshots: &mut Snapshots) -> Result<JobsPayload, Box<dyn Error>> {
        (self.scrape_fn)(snapshots).await
    }
}

pub async fn scrape_weedmaps(snapshots: &mut Snapshots) -> Result<JobsPayload, Box<dyn Error>> {
    let options = LaunchOptions {
        headless: false,
        window_size: Some((1920, 1080)),
        enable_logging: true,
        ..LaunchOptions::default()
    };
    let browser = Browser::new(options)?;
    let tab = browser.new_tab()?;
    tab.navigate_to("https://boards.greenhouse.io/embed/job_board?for=weedmaps77&b=https%3A%2F%2Fweedmaps.com%2Fcareers")?;
    tab.wait_for_element("body")?;

    // Evaluate JavaScript and parse the result
    let links = tab.evaluate(
        r#"
        JSON.stringify(Array.from(document.querySelectorAll('div[department_id="4069853002,4069854002"]')).map(job => {
            const link = job.querySelector("a").href;
            const [title, location] = job.innerText.split("\n");
            return {
                title,
                location,
                link
            };
        }))
        "#,
        false,
    )?;
    let scraped_jobs: Vec<Job> = serde_json::from_str(links.value.unwrap().as_str().unwrap())?;
    let old_jobs: HashSet<&str> = snapshots
        .weedmaps
        .iter()
        .map(|j| j.title.as_str())
        .collect();
    let new_jobs: Vec<Job> = scraped_jobs
        .iter()
        .filter(|&j| !old_jobs.contains(j.title.as_str()))
        .cloned()
        .collect();

    // Updatong snapshots
    snapshots.weedmaps = scraped_jobs.clone();
    snapshots.save();

    Ok(JobsPayload {
        are_new_jobs: !new_jobs.is_empty(),
        new_jobs,
        all_jobs: scraped_jobs,
    })
}
