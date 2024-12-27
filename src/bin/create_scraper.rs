use std::{fs, path::PathBuf};

use dialoguer::{Input, Select};
use jobshell::utils::clear_console;
use strum::IntoEnumIterator;
use strum_macros::{Display, EnumIter};

fn main() {
    clear_console();
    let company_name = Input::<String>::new()
        .with_prompt("Enter the name of the company (e.g. Google)")
        .interact()
        .unwrap();

    let sanitized_company_name = company_name.to_lowercase().replace(" ", "_");

    #[derive(EnumIter, Display)]
    enum ScraperType {
        #[strum(to_string = "Headless Chrome")]
        HeadlessChrome,

        #[strum(to_string = "Custom Scraper")]
        CustomScraper,
    }

    trait DisplayStrings {
        fn display_strings() -> Vec<String>;
    }

    impl DisplayStrings for ScraperType {
        fn display_strings() -> Vec<String> {
            ScraperType::iter().map(|st| st.to_string()).collect()
        }
    }

    let selected_scraper_option = Select::new()
        .with_prompt("Select the type of scraper")
        .items(&ScraperType::display_strings())
        .default(0)
        .interact()
        .unwrap();

    let module_string = format!(
        "\npub mod {sanitized_company_name} {{
	pub mod scraper;
}}\n"
    );
    let boilerplate_code = match ScraperType::iter().nth(selected_scraper_option).unwrap() {
        ScraperType::CustomScraper => {
            let boilerplate = format!(
                r#"
use std::error::Error;

use crate::models::{{
    data::Data,
    scraper::{{JobsPayload, ScrapedJob}},
}};

pub async fn scrape_{}(data: &mut Data) -> Result<JobsPayload, Box<dyn Error>> {{

		// Acquire Vector of ScrapedJob
    let scraped_jobs: Vec<ScrapedJob> =
        serde_json::from_str(remote_object.value.unwrap().as_str().unwrap()).unwrap();

		// Convert Vector of ScrapedJob into a JobsPayload
    let jobs_payload = JobsPayload::from_scraped_jobs(scraped_jobs, "{}", data);

	// Return JobsPayload
    Ok(jobs_payload)
}}
            "#,
                sanitized_company_name, company_name
            );

            boilerplate
        }

        ScraperType::HeadlessChrome => {
            let url = Input::<String>::new()
                .with_prompt("Enter the URL to scrape")
                .interact()
                .unwrap();

            let content_selector = Input::<String>::new()
                .with_prompt("Enter the CSS selector for the job listings")
                .allow_empty(true)
                .interact()
                .unwrap();

            let boilerplate = format!(
                r###"
                
use std::error::Error;

use headless_chrome::{{Browser, LaunchOptions}};

use crate::models::{{
    data::Data,
    scraper::{{JobsPayload, ScrapedJob}},
}};

pub async fn scrape_{}(data: &mut Data) -> Result<JobsPayload, Box<dyn Error>> {{
    let launch_options = LaunchOptions {{
        headless: false,
        window_size: Some((1920, 1080)),
        enable_logging: true,

        ..LaunchOptions::default()
    }};
    let browser = Browser::new(launch_options)?;

    let tab = browser.new_tab()?;

    tab.navigate_to("{}")?;
    tab.wait_for_element("body")?;
    {}
    

    let remote_object = tab.evaluate(
        r##"

// DELETE AND REPLACE WITH CUSTOM JS LOGIC    
const engJobs = document.querySelector("#jobs-16253")

const jobs = Array.from(engJobs.querySelectorAll(".job")).map(j => {{
    const title = j.querySelector(".job-title").innerHTML;
    const location = j.querySelector(".job-location").innerHTML;
    const link = j.querySelector("a").href;

    return {{
        title,
        location,
        link
    }}
}})

JSON.stringify(jobs);
    "##,
        false,
    )?;

		// Acquire Vector of ScrapedJob
    let scraped_jobs: Vec<ScrapedJob> =
        serde_json::from_str(remote_object.value.unwrap().as_str().unwrap()).unwrap();

		// Convert Vector of ScrapedJob into a JobsPayload
    let jobs_payload = JobsPayload::from_scraped_jobs(scraped_jobs, "{}", data);

	// Return JobsPayload
    Ok(jobs_payload)
}}
            "###,
                sanitized_company_name,
                url,
                {
                    if content_selector.is_empty() {
                        "".to_string()
                    } else {
                        format!(r#"tab.wait_for_element("{content_selector}")?;"#)
                    }
                },
                company_name
            );

            boilerplate
        }
    };

    const SCRAPERS_DIR: &str = "./src/scrapers";

    fn create_scraper_files(
        company_name: &str,
        boilerplate: &str,
        module_def: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        // Create the company-specific scraper directory
        let scraper_dir = PathBuf::from(SCRAPERS_DIR).join(company_name);
        fs::create_dir_all(&scraper_dir).map_err(|e| {
            format!(
                "Failed to create directory {}: {}",
                scraper_dir.display(),
                e
            )
        })?;

        // Create the scraper.rs file
        let scraper_file = scraper_dir.join("scraper.rs");
        fs::write(&scraper_file, boilerplate)
            .map_err(|e| format!("Failed to write {}: {}", scraper_file.display(), e))?;

        // Update mod.rs
        let mod_file = PathBuf::from(SCRAPERS_DIR).join("mod.rs");
        let mut mod_contents = fs::read_to_string(&mod_file).unwrap_or_default();

        // Only add the module declaration if it doesn't already exist
        if !mod_contents.contains(&format!("pub mod {}", company_name)) {
            mod_contents.push_str(module_def);
            fs::write(&mod_file, mod_contents)
                .map_err(|e| format!("Failed to update {}: {}", mod_file.display(), e))?;
        }

        Ok(())
    }

    // Create the files and handle any errors
    if let Err(e) = create_scraper_files(&sanitized_company_name, &boilerplate_code, &module_string)
    {
        eprintln!("Error creating scraper files: {}", e);
        std::process::exit(1);
    }

    println!("Scraper created successfully!");
}
