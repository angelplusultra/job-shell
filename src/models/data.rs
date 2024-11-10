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
pub struct DataV2 {
    pub data: HashMap<String, Company>,
}

impl DataV2 {
    pub fn default() -> Self {
        let companies: Vec<(String, Company)> = COMPANYKEYS
            .iter()
            .map(|&v| (v.to_string(), Company::new()))
            .collect();
        DataV2 {
            data: HashMap::from_iter(companies),
        }
    }
    pub fn save(&self) {
        let data = json!({
            "data": self
        });

        fs::write("data.json", data.to_string()).expect("Error writing to data.json");
    }

    pub fn get_data() -> DataV2 {
        let default_data = DataV2::default();

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
                        serde_json::from_value::<DataV2>(data.clone())
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

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct Data {
    pub weedmaps: Company,
    pub onepassword: Company,
    pub square: Company,
    pub tarro: Company,
    pub anduril: Company,
}

impl Data {
    pub fn default() -> Self {
        Data {
            anduril: Company::new(),
            weedmaps: Company::new(),
            onepassword: Company::new(),
            tarro: Company::new(),
            square: Company::new(),
        }
    }

    pub fn save(&self) {
        let data = json!({
            "data": self
        });

        fs::write("data.json", data.to_string()).expect("Error writing to snapshots.json");
    }

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
