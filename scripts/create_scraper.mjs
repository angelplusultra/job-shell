import asyncFs from "fs/promises";
import fs from "fs";
import readline from "readline";
import path from "path";
import { dirname } from "path";
import { fileURLToPath } from "url";

// Convert the module's URL to a file path
const __filename = fileURLToPath(import.meta.url);

// Get the directory name of the current module
const __dirname = dirname(__filename);

const rl = readline.createInterface({
  input: process.stdin,
  output: process.stdout,
});

const { companyName, url, contentSelector } = await new Promise((res) => {
  rl.question("What is the name of the company? ", (companyName) => {
    companyName = companyName.split(" ").join("_");
    companyName = companyName.split(",").join("_");

    rl.question("What is the careers URL? ", (url) => {
      rl.question(
        `What is the content selector (e.g ".content")? `,
        (contentSelector) => {
          res({
            companyName,
            url,
            contentSelector,
          });
        },
      );
    });
  });
});
rl.close();

/**
 * @param {string} name
 *	@param {string} url - the name
 *	@param {string} content_selector
 *
 */
function create_new_scraper(name, url, content_selector) {
  const dataKey = name[0].toUpperCase() + name.slice(1).toLowerCase();

  const rustCode = `
use std::error::Error;

use headless_chrome::{Browser, LaunchOptions};

use crate::models::{
    data::Data,
    scraper::{JobsPayload, ScrapedJob},
};

pub async fn scrape_${name.toLowerCase()}(data: &mut Data) -> Result<JobsPayload, Box<dyn Error>> {
    let launch_options = LaunchOptions {
        headless: false,
        window_size: Some((1920, 1080)),
        enable_logging: true,

        ..LaunchOptions::default()
    };
    let browser = Browser::new(launch_options)?;

    let tab = browser.new_tab()?;

    tab.navigate_to("${url}")?;
    tab.wait_for_element("body")?;
    tab.wait_for_element("${content_selector || `body`}")?;

    let scraped_jobs = tab.evaluate(
        r##"

// DELETE AND REPLACE WITH CUSTOM JS LOGIC    
const engJobs = document.querySelector("#jobs-16253")

const jobsPayload = Array.from(engJobs.querySelectorAll(".job")).map(j => {
    const title = j.querySelector(".job-title").innerHTML;
    const location = j.querySelector(".job-location").innerHTML;
    const link = j.querySelector("a").href;

    return {
        title,
        location,
        link
    }
})

JSON.stringify(jobsPayload);
    "##,
        false,
    )?;

    let scraped_jobs: Vec<ScrapedJob> =
        serde_json::from_str(scraped_jobs.value.unwrap().as_str().unwrap()).unwrap();

    let jobs_payload = JobsPayload::from_scraped_jobs(scraped_jobs, &data.data["${dataKey}"]);

    data.data.get_mut("${dataKey}").unwrap().jobs = jobs_payload.all_jobs.clone();

    data.save();

    Ok(jobs_payload)
}


	`;

  const rustCodeDeprecated = `async fn scrape_${name.toLowerCase()}(data: &mut Data) -> Result<JobsPayload, Box<dyn Error>> {
      let launch_options = LaunchOptions {
        headless: options.headless,
        window_size: Some((1920, 1080)),
        enable_logging: true,

        ..LaunchOptions::default()
    };
    let browser = Browser::new(launch_options)?;

    let tab = browser.new_tab()?;

    tab.navigate_to("${url}")?;
    tab.wait_for_element("body")?;
    tab.wait_for_element("${content_selector || `body`}")?;

	
/*
		--------IMPLEMENT CUSTOM LOGIC TO SCRAPE JOBS---------
		
 		let engineering_jobs = tab.evaluate(&options.get_jobs_js, false)?;

    let scraped_jobs: Vec<ScrapedJob> =
        serde_json::from_str(engineering_jobs.value.unwrap().as_str().unwrap()).unwrap();

    let onepassword_jobs_payload =
        JobsPayload::from_scraped_jobs(scraped_jobs, &data.data[options.company_key]);

    data.data.get_mut(options.company_key).unwrap().jobs =
        onepassword_jobs_payload.all_jobs.clone();

    data.save();

    Ok(onepassword_jobs_payload)
*/

	Err(())

  }`;

  const scrapersDir = path.join(__dirname, "..", "src", "scrapers");
  const companyDir = path.join(scrapersDir, name.toLowerCase());
  const fullFilePath = path.join(companyDir, "scraper.rs");

  const moduleData = `pub mod ${name.toLowerCase()} {
	pub mod scraper;
}`;

  return {
    scrapersDir,
    companyDir,
    rustCode,
    fullFilePath,
    moduleData,
  };
}

const scraper_data = create_new_scraper(companyName, url, contentSelector);
const modPath = path.join(__dirname, "..", "src", "scrapers", "mod.rs");
try {
  const pathExists = fs.existsSync(scraper_data.fullFilePath);
  if (!pathExists) {
    await asyncFs.mkdir(scraper_data.companyDir);
  }
  await asyncFs.writeFile(scraper_data.fullFilePath, scraper_data.rustCode);

  await asyncFs.appendFile(modPath, scraper_data.moduleData);
} catch (error) {
  console.log(error);
}
