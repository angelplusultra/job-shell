use std::{error::Error, str::FromStr};

use anyhow::anyhow;
use reqwest::Client;
use serde_json::{json, Value};
use uuid::Uuid;

use super::scraper::Job;

pub trait AiModel {
    fn new() -> Self;
    async fn generate_response(
        &self,
        system_prompt: &str,
        user_prompt: &str,
        response_format: Value,
    ) -> Result<Value, Box<dyn Error + Send + Sync>>;
}

pub struct OpenAIClient {
    token: String,
    client: Client,
}

impl AiModel for OpenAIClient {
    fn new() -> Self {
        let token = std::env::var("OPENAI_KEY").expect("OPENAI_KEY must be set");

        Self {
            token,
            client: Client::new(),
        }
    }

    async fn generate_response(
        &self,
        system_prompt: &str,
        user_prompt: &str,
        response_format: Value,
    ) -> Result<Value, Box<dyn Error + Send + Sync>> {
        let json: Value = self
            .client
            .post("https://api.openai.com/v1/chat/completions")
            .bearer_auth(&self.token)
            .json(&json!(
                {
                  "model": "gpt-4o",
                  "messages": [
                    {
                      "role": "system",
                      "content": system_prompt
                    },
                    {
                      "role": "user",
                      "content": user_prompt
                    }
                  ],
                  "response_format": {
                    "type": "json_schema",
                    "json_schema": response_format
                  }
                }
            ))
            .send()
            .await?
            .json()
            .await?;
        Ok(json)
    }
}

impl OpenAIClient {
    pub async fn filter_jobs_based_on_smart_criteria(
        &self,
        jobs: &Vec<Job>,
        smart_criteria: &str,
    ) -> Result<Vec<Job>, Box<dyn Error + Send + Sync>> {
        let system_prompt = r#"
        You are a JSON processing assistant. Your job is to read a single string of text instructions—called "criteria"—and use it to filter an array of jobs, returning only those that match the user's criteria. Follow these instructions carefully:

1. **Input Format**:
   - You will receive an object with two properties:  
     - `criteria`: A string describing which jobs the user wants.  
     - `jobs`: An array of job objects, each containing properties like `id`, `title` and `location`.

2. **Interpretation & Filtering**:
   - Parse the text in `criteria` to understand what the user is looking for (e.g., desired job titles, locations, keywords, etc.).
   - Filter the `jobs` array accordingly, returning only the jobs that match the user’s instructions.  
     - For instance, if `criteria` is:  
       `"I am interested in any Software Engineer jobs Remote, US or based in any city in Southern California."`  
       You should keep jobs with a `title` containing "Software Engineer" and a `location` that suggests remote or a location in the US or southern California.

3. **Output Format**:
   - Return the filtered jobs as a **raw JSON array** of the matching job ID strings.
   - Example of expected output:  
     ```
     [
        "b1b50f3a-4b26-4a92-af43-c4747a416c33",
        "b1b50f3a-4b26-4a92-af43-c4747a416c34",
        "b1b50f3a-4b26-4a92-af43-c4747a416c35"
         
     ]
     ```

    - If you are confident that no jobs match the criteria, return an empty array `[]`.

   - The output must be valid JSON with no additional text, code blocks, or surrounding formatting.

4. **Guidelines**:
   - Do not add comments, markdown, or explanations in the output.
   - Do not include any symbols or text other than the raw JSON array.
   - The output should be directly usable in a JavaScript program.
        "#;

        let formatted_jobs = jobs
            .iter()
            .map(|j| {
                json!({
                    "id": j.id,
                    "title": j.title,
                    "location": j.location
                })
            })
            .collect::<Vec<Value>>();

        let user_prompt = json!({
            "criteria": smart_criteria,
            "jobs": formatted_jobs
        });

        let response_format = json!({
                      "name": "job_ids",
                      "schema": {
                          "type": "object",
                          "properties": {
                              "job_ids": {
                              "type": "array",
                              "items": {
                                  "type": "string"
                              }
                          }
                      },

              "required": ["job_ids"],
          "additionalProperties": false,
          },
        "strict": true




          });

        let json_response = self
            .generate_response(system_prompt, &user_prompt.to_string(), response_format)
            .await
            .map_err(|_| anyhow!("Error generating response")).unwrap();

        let serialied_text_response = json_response["choices"][0]["message"]["content"]
            .as_str()
            .ok_or("Invalid response").unwrap();

        let deserialized_text_response: Value = serde_json::from_str(serialied_text_response).unwrap();

        let job_ids: Vec<Uuid> = deserialized_text_response["job_ids"]
            .as_array()
            .unwrap()
            .iter()
            .map(|v| Uuid::from_str(v.as_str().unwrap()).unwrap())
            .collect();

        let filtered_jobs: Vec<Job> = jobs
            .iter()
            .filter(|j| job_ids.contains(&j.id))
            .cloned()
            .collect();

        Ok(filtered_jobs)
    }
}
