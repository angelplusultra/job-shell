use std::{collections::HashMap, error::Error, fs};

use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

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
            "data": self.data
        });

        fs::write("data.json", serde_json::to_string_pretty(&data).unwrap())
            .expect("Error writing to data.json");
    }

    fn process_data() -> Result<Data, Box<dyn Error>> {
        // Read and parse the file
        let content = fs::read_to_string("data.json")?;
        let mut json_value: Value = serde_json::from_str(&content)?;

        // Get mutable reference to the data object
        let data_obj = json_value
            .get_mut("data")
            .ok_or("Missing 'data' field")?
            .as_object_mut()
            .ok_or("'data' field is not an object")?;

        // Track if we made any changes
        let mut made_changes = false;

        // Check and add missing company keys
        for key in COMPANYKEYS {
            if !data_obj.contains_key(key) {
                data_obj.insert(
                    key.to_string(),
                    json!({
                        "connections": [],
                        "jobs": []
                    }),
                );
                made_changes = true;
            }
        }

        // If we made changes, save the updated data
        if made_changes {
            fs::write("data.json", serde_json::to_string_pretty(&json_value)?)?;
        }

        // Convert to your Data type
        let data: Data = serde_json::from_value(json_value)?;
        Ok(data)
    }
    pub fn get_data() -> Data {
        let default_data = Self::default();

        match Self::process_data() {
            Ok(data) => data,
            Err(e) => {
                eprintln!("{}", e);
                let default = default_data;
                default.save();
                default
            }
        }
    }
    // TODO: Need to check COMPANY_KEYS for new keys and save if new keys are there.
    // pub fn get_data() -> Data {
    //     let default_data = Data::default();
    //
    //     fs::read_to_string("data.json")
    //         .map_err(|_| "Failed to read snapshots file")
    //         .and_then(|content| {
    //             serde_json::from_str::<serde_json::Value>(&content)
    //                 .map_err(|_| "Failed to parse JSON")
    //         })
    //         .and_then(|mut value| {
    //             value
    //                 .get_mut("data")
    //                 .ok_or("Missing 'data' field")
    //                 .and_then(|data| {
    //                     let mut new_company_keys_detected = false;
    //                     for key in COMPANYKEYS {
    //                         if !data["data"].as_object().unwrap().contains_key(key) {
    //                             data["data"][key] = json!({
    //                                 "connections": [],
    //                                 "jobs": []
    //                             });
    //                             new_company_keys_detected = true;
    //                         }
    //                         if new_company_keys_detected {
    //
    //                         }
    //                     }
    //                     let data = serde_json::from_value::<Data>(data.clone()).expect("IDK some shit failed");
    //
    //                     data.save();
    //
    //
    //                 })
    //         })
    //         .unwrap_or_else(|err| {
    //             eprintln!("Error reading snapshots: {}", err);
    //             default_data.save();
    //             default_data
    //         })
    // }
}
