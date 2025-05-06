use anyhow::Error;
use anyhow::Result;
use serde_json::Value;
use std::fs;
use std::path::PathBuf;
use walkdir::WalkDir;

pub struct TestCase {
    pub path: String,
    pub description: String,
    pub source: String,
    pub json: String,
    pub remainder: String,
}

pub fn get_file_list(
    source_dir: &PathBuf,
    extensions: &Vec<String>,
) -> Result<Vec<PathBuf>> {
    let walker = WalkDir::new(source_dir).into_iter();
    let files = walker
        .filter_map(|path_result| match path_result {
            Ok(path) => match path.path().extension() {
                Some(ext) => {
                    if extensions.contains(&ext.display().to_string()) {
                        Some(path.path().to_path_buf())
                    } else {
                        None
                    }
                }
                None => None,
            },
            Err(_) => None,
        })
        .collect();
    Ok(files)
}

pub fn get_test_data(source_path: &PathBuf) -> Result<TestCase> {
    let content = fs::read_to_string(source_path)?;
    let parts: Vec<_> = content
        .split("~~~~~~")
        .filter_map(|part| Some(part.trim_end().to_string()))
        .collect();
    if parts.len() == 4 {
        let remainder_json: Value = serde_json::from_str(&parts[3]).unwrap();
        Ok(TestCase {
            path: source_path.display().to_string(),
            description: parts[1].clone(),
            source: parts[0].clone(),
            json: parts[2].clone(),
            remainder: remainder_json.get("remainder").unwrap().to_string(),
        })
    } else {
        Err(Error::msg(format!(
            "malformed test file: {}",
            source_path.display()
        )))
    }
}
