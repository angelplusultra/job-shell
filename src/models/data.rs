use std::{collections::HashMap, fs};

use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::COMPANYKEYS;

use super::scraper::Job;

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct Company {
    pub connections: Vec<Conenction>,
    pub jobs: Vec<Job>,
}

impl Company {
    pub fn new() -> Self {
        Company {
            jobs: Vec::new(),
            connections: Vec::new(),
        }
    }
}

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct Conenction {
    pub first_name: String,
    pub last_name: String,
    pub email: Option<String>,
    pub linkedin: Option<String>,
}

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct Data {
    pub data: HashMap<String, Company>,
}

impl Data {
    pub fn default() -> Self {
        let companies: Vec<(String, Company)> = COMPANYKEYS
            .iter()
            .map(|&v| (v.to_string(), Company::new()))
            .collect();
        Data {
            data: HashMap::from_iter(companies),
        }
    }
    pub fn save(&self) {
        let data = json!({
            "data": self
        });

        fs::write("data.json", data.to_string()).expect("Error writing to data.json");
    }

    // TODO: Need to check COMPANY_KEYS for new keys and save if new keys are there.
    pub fn get_data() -> Data {
        let default_data = Data::default();

        fs::read_to_string("data.json")
            .map_err(|_| "Failed to read snapshots file")
            .and_then(|content| {
                serde_json::from_str::<serde_json::Value>(&content)
                    .map_err(|_| "Failed to parse JSON")
            })
            .and_then(|value| {
                value
                    .get("data")
                    .ok_or("Missing 'data' field")
                    .and_then(|data| {
                        serde_json::from_value::<Data>(data.clone())
                            .map_err(|_| "Failed to parse Snapshots")
                    })
            })
            .unwrap_or_else(|err| {
                eprintln!("Error reading snapshots: {}", err);
                default_data.save();
                default_data
            })
    }
}

