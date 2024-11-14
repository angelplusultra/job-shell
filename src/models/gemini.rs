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
    // index: u32,
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
        let original_prompt = r#"Please respond with a json object in the following format. DO NOT respond with markdown, respond with raw json that adheres to the following structure:


{}


I REPEAT, DO NOT RESPOND WITH ANYTHING BESDIES RAW STRING JSON, DO NOT WRAP THE JSON IN BACKTICKS, Do not do this:  ```json```


DO NOT WRAP IN BACKTICKS, DO NOT WRAP IN BACKTICKS, DO NOT WRAP IN BACKTICKS, RAW JSON STRING ONLY. MY LIFE DEPENDS ON IT, YOU CANNOT MESS THIS UP.

Another Important Details:"#;

        let gemini_client = GeminiClient::new();
        let s = format!(
            r#"{} 

            Prompt: Please parse the HTML Job page and return the job details. The JSON structure is as specified.

            Do not exceed 500 characters for the job_description field.

Another Important Details:

If you do not have any data for a field just put ["NOT SPECIFIED"] for the ARRAY types and "NOT SPECIFIED" for the STRING types instead of null
"#,
            &html
        );

        // let response = gemini_client
        //     .client
        //     .post(&gemini_client.url_with_api_key)
        //     .json(&json!({
        //         "contents": {
        //         "parts": [{"text": s}]
        //     }
        //     }))
        //     .send()
        //     .await?
        //     .json::<Root>()
        //     .await?;

        // TODO: Refactor this bullshit to propogate errors upwards naturally with (?)
        match gemini_client
            .client
            .post(&gemini_client.url_with_api_key)
            .json(&json!({
                        "generationConfig": {
                "response_mime_type": "application/json",
                "response_schema": {
                  "type": "OBJECT",
                    "properties": {
                      "title": {"type":"STRING"},
                      "skills": {
                            "type":"ARRAY",
                            "items": {
                                "type": "STRING"
                        }
                    },
                      "job_description": {"type":"STRING"},
                      "years_of_experience": {"type":"STRING"},
                      "compensation": {"type":"STRING"},
                      "benefits": {
                            "type":"ARRAY",
                            "items": {
                                "type": "STRING"
                        }
                    },
                      "location": {"type":"STRING"},
                    }
                }
            },
                        "contents": {
                        "parts": [{"text": s}]
                    }
                    }))
            .send()
            .await
        {
            Ok(res) => match res.json::<Value>().await {
                Ok(json) => {
                    println!("{:#?}", json);
                    let json_response =
                        json["candidates"][0]["content"]["parts"][0]["text"].clone();

                    let gemini_job = serde_json::from_str::<Self>(json_response.as_str().unwrap())?;
                    return Ok(gemini_job);
                }
                Err(e) => {
                    eprintln!("Error: {e}");
                    return Err(Box::new(e));
                }
            },
            Err(e) => {
                eprintln!("Error: {e}");
                return Err(Box::new(e));
            }
        }

        // let job_obj: GeminiJob = serde_json::from_str(j).unwrap();
        // let response_json = &response.candidates[0].content.parts[0].text;
        //
        // if let Ok(job) = serde_json::from_str::<Self>(response_json) {
        //     return Ok(job);
        // } else {
        //     return Err(Box::new(CustomError {
        //         details: "The html couldnt be parsed".to_string(),
        //     }));
        // }
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
    url: String,
    key: String,
    url_with_api_key: String,
    client: Client,
}

impl GeminiClient {
    pub fn new() -> Self {
        let url = format!("https://generativelanguage.googleapis.com/v1beta/models/gemini-1.5-{}:generateContent", env::var("GEMINI_MODEL").expect("GEMINI_MODEL is missing"));
        let key = env::var("GEMINI_KEY").expect("GEMINI_KEY is missing");
        let url_with_api_key = format!("{url}?key={key}");

        GeminiClient {
            url,
            key,
            url_with_api_key,
            client: Client::new(),
        }
    }
}
