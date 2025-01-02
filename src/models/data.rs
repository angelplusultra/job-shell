use std::{
    collections::{HashMap, HashSet},
    error::Error,
    fs,
    path::PathBuf,
};

use dialoguer::{theme::ColorfulTheme, Confirm, Input};
use directories::ProjectDirs;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use tabled::Tabled;

use crate::COMPANYKEYS;

use super::scraper::Job;

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct Company {
    pub connections: Vec<Connection>,
    pub jobs: Vec<Job>,
    pub is_following: bool,
}

impl Company {
    pub fn new() -> Self {
        Company {
            jobs: Vec::new(),
            connections: Vec::new(),
            is_following: false,
        }
    }
}

#[derive(Debug, Default, Deserialize, Serialize, Tabled, Clone)]
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
        None => "N/A", // Customize for missing companies
    }
}

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct Data {
    pub companies: HashMap<String, Company>,
    pub smart_criteria: String,
    pub smart_criteria_enabled: bool
}

#[derive(Debug)]
pub struct JobCounts {
    intern: i32,
    junior: i32,
    mid: i32,
    senior: i32,
    staff: i32,
    principal: i32,
    unidentified: i32,
}

impl JobCounts {
    pub fn export_csv(&self) -> Result<(), Box<dyn Error>> {
        let csv_string = format!("Title,Quantity\nIntern,{}\nJunior,{}\nMid,{}\nSenior,{}\nStaff,{}\nPrincipal,{}\nUnidentified,{}", self.intern, self.junior, self.mid, self.senior, self.staff, self.principal, self.unidentified);
        fs::write("./job_data.csv", csv_string)?;

        Ok(())
    }
}
pub trait AnalyzeData {
    fn get_job_counts(&self) -> JobCounts;
}

impl AnalyzeData for Data {
    fn get_job_counts(&self) -> JobCounts {
        let intern_tokens: HashSet<&str> = HashSet::from_iter(vec!["intern", "internship"]);
        let junior_tokens: HashSet<&str> = HashSet::from_iter(vec![
            "junior",
            "i",
            "new grad",
            "new graduate",
            "newgrad",
            "entry",
            "jr",
            "jr.",
            "entry",
        ]);

        let mid_tokens: HashSet<&str> = HashSet::from_iter(vec!["mid", "ii"]);
        let senior_tokens: HashSet<&str> = HashSet::from_iter(vec!["senior", "iii", "sr", "sr."]);
        let staff_tokens: HashSet<&str> = HashSet::from_iter(vec!["staff", "iv"]);
        let principal_tokens: HashSet<&str> = HashSet::from_iter(vec!["principal"]);
        let mut intern = 0;
        let mut junior = 0;
        let mut senior = 0;
        let mut staff = 0;
        let mut mid = 0;
        let mut principal = 0;
        let mut unidentified = 0;
        for (_, c) in self.companies.iter() {
            'job_loop: for j in c.jobs.iter() {
                let normalized_title = j.title.to_lowercase();

                let tokens = normalized_title.split_whitespace().collect::<Vec<&str>>();

                for token in tokens.iter() {
                    if intern_tokens.contains(token) {
                        intern += 1;
                        continue 'job_loop;
                    }
                    if junior_tokens.contains(token) {
                        junior += 1;
                        continue 'job_loop;
                    }

                    if mid_tokens.contains(token) {
                        mid += 1;
                        continue 'job_loop;
                    }
                    if senior_tokens.contains(token) {
                        senior += 1;
                        continue 'job_loop;
                    }

                    if staff_tokens.contains(token) {
                        staff += 1;
                        continue 'job_loop;
                    }

                    if principal_tokens.contains(token) {
                        principal += 1;
                        continue 'job_loop;
                    }

                    unidentified += 1;
                }
            }
        }

        return JobCounts {
            intern,
            mid,
            staff,
            senior,
            junior,
            principal,
            unidentified,
        };
    }
}

impl Data {
    pub fn default() -> Self {
        let companies: Vec<(String, Company)> = COMPANYKEYS
            .iter()
            .map(|&v| (v.to_string(), Company::new()))
            .collect();
        Data {
            companies: HashMap::from_iter(companies),
            smart_criteria: "".to_string(),
            smart_criteria_enabled: false
        }
    }
    pub fn save(&self) {
        let data = json!({
            "companies": self.companies,
            "smart_criteria": self.smart_criteria,
            "smart_criteria_enabled": self.smart_criteria_enabled
        });

        let data_file_path = Self::get_data_dir().join("data.json");
        fs::write(data_file_path, serde_json::to_string_pretty(&data).unwrap())
            .expect("Error writing to data.json");
    }

    pub fn get_data_dir() -> PathBuf {
        let project_dir = ProjectDirs::from("org", "jobshell", "jobshell")
            .expect("Problem configuring the project directory");

        project_dir.data_dir().to_path_buf()
    }
    fn process_data() -> Result<Data, Box<dyn Error>> {
        let data_dir = Data::get_data_dir();

        if !fs::exists(&data_dir)? {
            fs::create_dir_all(&data_dir)?;
        }

        let data_file = data_dir.join("data.json");
        // Read and parse the file
        let content = fs::read_to_string(&data_file)?;
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
            fs::write(&data_file, serde_json::to_string_pretty(&json_value)?)?;
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
            .companies
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
            .companies
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

    pub fn toggle_company_follow(&mut self, company_key: &'static str) {
        let c = self.companies.get_mut(company_key).unwrap();

        c.is_following = !c.is_following;

        self.save();
    }

    pub fn toggle_job_bookmark(&mut self, id: &uuid::Uuid) {
        if self
            .companies
            .iter_mut()
            .flat_map(|(_, c)| &mut c.jobs)
            .any(|j| {
                if j.id == *id {
                    j.is_bookmarked = !j.is_bookmarked;
                    true
                } else {
                    false
                }
            })
        {
            self.save();
        }
    }
}
