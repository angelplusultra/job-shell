use std::fs;

use serde::{Deserialize, Serialize};
use serde_json::json;

use super::scraper::Job;

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct Snapshots {
    pub weedmaps: Vec<Job>,
    pub onepassword: Vec<Job>,
    pub square: Vec<Job>,
    pub tarro: Vec<Job>,
    pub anduril: Vec<Job>
}

impl Snapshots {
    pub fn default() -> Self {
        Snapshots {
            weedmaps: Vec::new(),
            onepassword: Vec::new(),
            square: Vec::new(),
            tarro: Vec::new(),
            anduril: Vec::new(),
        }
    }

    pub fn save(&self) {
        let data = json!({
            "data": self
        });

        fs::write("snapshots.json", data.to_string()).expect("Error writing to snapshots.json");
    }

    pub fn get_snapshots() -> Snapshots {
        let default_snapshots = Snapshots::default();

        fs::read_to_string("snapshots.json")
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
                        serde_json::from_value::<Snapshots>(data.clone())
                            .map_err(|_| "Failed to parse Snapshots")
                    })
            })
            .unwrap_or_else(|err| {
                eprintln!("Error reading snapshots: {}", err);
                default_snapshots.save();
                default_snapshots
            })
    }
}
