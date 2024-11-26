use std::{collections::HashMap, error::Error, fs};

use dialoguer::{theme::ColorfulTheme, Confirm, Input};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use tabled::Tabled;

use crate::COMPANYKEYS;

use super::scraper::Job;

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct Company {
    pub connections: Vec<Connection>,
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

#[derive(Debug, Default, Deserialize, Serialize, Tabled)]
pub struct Connection {
    pub first_name: String,
    pub last_name: String,
    pub company: String,
    pub current_employee: bool,
    pub role: String,
    #[tabled(display_with = "display_option")]
    pub email: Option<String>,
    #[tabled(display_with = "display_option")]
    pub linkedin: Option<String>,
}

impl Connection {
    pub fn create_with_form(dialoguer_styles: &ColorfulTheme, company: &str) -> Self {
        let first_name: String = Input::with_theme(dialoguer_styles)
            .with_prompt("Enter their first name")
            .interact_text()
            .unwrap();

        let last_name: String = Input::with_theme(dialoguer_styles)
            .with_prompt("Enter their last name")
            .interact_text()
            .unwrap();

        let current_employee = Confirm::with_theme(dialoguer_styles)
            .with_prompt("Are they currently employed at this company?")
            .interact()
            .unwrap();

        let role: String = Input::with_theme(dialoguer_styles)
            .with_prompt("Enter their role at this company (e.g Software Engineer)")
            .interact_text()
            .unwrap();

        let email: Option<String> = Input::with_theme(dialoguer_styles)
            .with_prompt("Enter their email (Press Enter to skip)")
            .allow_empty(true)
            .interact_text()
            .ok()
            .filter(|s: &String| !s.is_empty());

        let linkedin: Option<String> = Input::with_theme(dialoguer_styles)
            .with_prompt("Enter their LinkedIn profile (Press Enter to skip)")
            .with_initial_text("https://linkedin.com/in/")
            .allow_empty(true)
            .validate_with(|c: &String| {
                if !c.starts_with("https://linkedin.com/in/") {
                    Err("This is not the valid schema for a linkedin profile href")
                } else {
                    Ok(())
                }
            })
            .interact_text()
            .ok()
            .filter(|s: &String| {
                if s == "https://linkedin.com/in/" {
                    return false;
                }

                return true;
            });

        Connection {
            first_name,
            last_name,
            company: company.to_string(),
            role,
            current_employee,
            email,
            linkedin,
        }
    }
}

// Helper function for displaying Option<String>
fn display_option(opt: &Option<String>) -> &str {
    match opt {
        Some(value) => value.as_str(),
        None => "N/A", // Customize for missing data
    }
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

    pub fn mark_job_seen(&mut self, id: &uuid::Uuid) {
        if self
            .data
            .iter_mut()
            .flat_map(|(_, c)| &mut c.jobs)
            .any(|j| {
                if j.id == *id {
                    j.is_seen = true;
                    true
                } else {
                    false
                }
            })
        {
            self.save();
        }
    }

    pub fn mark_job_applied(&mut self, id: &uuid::Uuid) {
        if self
            .data
            .iter_mut()
            .flat_map(|(_, c)| &mut c.jobs)
            .any(|j| {
                if j.id == *id {
                    j.applied = true;
                    true
                } else {
                    false
                }
            })
        {
            self.save();
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
