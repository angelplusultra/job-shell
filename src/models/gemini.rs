use std::{env, error::Error};

use reqwest::Client;
use serde::Deserialize;
use serde_json::{json, Value};

use crate::models::scraper::Job;

use super::{custom_error::CustomError, data::Data, scraper::JobsPayload};

#[derive(Debug, Deserialize)]
pub struct Root {
    pub candidates: Vec<Candidate>,
    modelVersion: String,
    usageMetadata: UsageMetadata,
}

#[derive(Debug, Deserialize)]
pub struct Candidate {
    pub content: Content,
    finishReason: String,
    index: u32,
    safetyRatings: Vec<SafetyRating>,
}

#[derive(Debug, Deserialize)]
pub struct Content {
    pub parts: Vec<Part>,
    role: String,
}

#[derive(Debug, Deserialize)]
pub struct Part {
    pub text: String,
}

#[derive(Debug, Deserialize)]
pub struct SafetyRating {
    category: String,
    probability: String,
}

#[derive(Debug, Deserialize)]
pub struct UsageMetadata {
    candidatesTokenCount: u32,
    promptTokenCount: u32,
    totalTokenCount: u32,
}

#[derive(Debug, Deserialize)]
pub struct GeminiJob {
    pub title: String,
    pub job_description: String,
    pub years_of_experience: String,
    pub compensation: String,
    pub location: String,
    pub skills: Vec<String>,
    pub benefits: Vec<String>,
}

impl GeminiJob {
    pub fn print_job(&self) {
        println!();
        println!("Title: {}", self.title);
        println!();
        println!("Summary: {}", self.job_description);
        println!();
        println!("Skills: {}", self.skills.join(","));
        println!();
        println!("Years of XP: {}", self.years_of_experience);
        println!();
        println!("Compensation: {}", self.compensation);
        println!();
        println!("Benefits: {}", self.benefits.join(","));
        println!();
        println!("Location: {}", self.location);
        println!();
    }

    pub async fn from_job_html(html: String) -> Result<Self, Box<dyn Error>> {
        let gemini_client = GeminiClient::new();
        let s = format!(
            r#"{} Prompt: Please respond with a json object in the following format. DO NOT respond with markdown, respond with raw json that adheres to the following structure:


{}


I REPEAT, DO NOT RESPOND WITH ANYTHING BESDIES RAW STRING JSON, DO NOT WRAP THE JSON IN BACKTICKS.

Another Important Details:

If you do not have any data for a field just put "NOT SPECIFIED" or ["NOT SPECIFIED"] instead of null
"#,
            &html, GEMINI_JSON
        );

        // "AIzaSyAJLcuradb-Q6XuwrWuKA0HdST6sWbYMAY";
        let response = gemini_client
            .client
            .post(&gemini_client.url_with_api_key)
            .json(&json!({
                "contents": {
                "parts": [{"text": s}]
            }
            }))
            .send()
            .await?
            .json::<Root>()
            .await?;

        // let job_obj: GeminiJob = serde_json::from_str(j).unwrap();
        let response_json = &response.candidates[0].content.parts[0].text;

        if let Ok(job) = serde_json::from_str::<Self>(response_json) {
            return Ok(job);
        } else {
            return Err(Box::new(CustomError {
                details: "The html couldnt be parsed".to_string(),
            }));
        }
    }
}

pub const GEMINI_JSON: &'static str = r#"{
    "title": "Software Engineer III",
    "job_description": "The summary of the job",
    "skills": ["React", "NodeJS", "AWS"],
    "years_of_experience": "5-7 years",
    "compensation": "$100,000 - $150,000"
    "benefits": ["401k Match", "Equity"],
    "location": "Remote"
}"#;

pub struct GeminiClient {
    url: &'static str,
    key: String,
    url_with_api_key: String,
    client: Client,
}

impl GeminiClient {
    pub fn new() -> Self {
        let url = "https://generativelanguage.googleapis.com/v1beta/models/gemini-1.5-flash:generateContent";
        let key = env::var("GEMINI_KEY").expect("GEMINI_KEY is missing");
        let url_with_api_key = format!("{url}?key={key}");

        GeminiClient {
            url,
            key,
            url_with_api_key,
            client: Client::new(),
        }
    }

    pub async fn get_jobs_payload_from_html(
        &self,
        html: &String,
        snapshots: &mut Data,
    ) -> Result<JobsPayload, Box<dyn Error>> {
        let prompt = format!(
            r#"
    Prompt: Please parse the jobs in this HTML and return a JSON string in the following format:

    "{{"jobs": [{{"title": "Software Engineer III","location": "Los Angeles, CA (Remote)","link": "https://thehreftoapply.com"}}]}}"


    IMPORTANT: Do not format the JSON in a markdown style with backticks, for example do not do this: 

    ```json

    ```


    Just return raw string JSON and DO NOT use any newline characters (\n) or (\t) or control characters. It should be a contiguous string.

    DO NOT INCLUDE (\\u0000-\\u001F)



    HTML: {}
    "#,
            html
        );
        let response = self
            .client
            .post(self.url_with_api_key.as_str())
            .json(&json!({
                "contents": {
                "parts": [{"text": prompt}]
            }
            }))
            .send()
            .await?
            .json::<Root>()
            .await?;

        let job_data: Value = serde_json::from_str(&response.candidates[0].content.parts[0].text)?;

        let jobs: Vec<Job> = serde_json::from_value(job_data["jobs"].clone())?;

        let jobs_payload = JobsPayload::from_jobs(&jobs, &snapshots.tarro.jobs);

        Ok(jobs_payload)
    }
}
