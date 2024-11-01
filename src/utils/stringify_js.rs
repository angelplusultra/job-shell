use std::{
    error::Error,
    fs,
    path::{Path, PathBuf},
};

pub fn strinfify_js(file_path: PathBuf) -> Result<String, Box<dyn Error>> {
    let js = fs::read_to_string(file_path)?;

    Ok(js)
}
