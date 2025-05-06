use anyhow::Result;
use serde_json::Value;
use std::fs;
use std::path::PathBuf;
use walkdir::WalkDir;

pub enum TestCase {
    Ok {
        description: String,
        json: String,
        path: String,
        remainder: String,
        source: String,
    },
    Err {
        description: String,
        path: String,
        source: String,
    },
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

pub fn get_test_data(source_path: &PathBuf) -> TestCase {
    let content = fs::read_to_string(source_path).unwrap();
    let parts: Vec<_> = content
        .split("~~~~~~")
        .filter_map(|part| Some(part.trim_end().to_string()))
        .collect();
    let source = match parts[1].as_str() {
        "json" => {
            let json: Value = serde_json::from_str(&parts[0]).unwrap();
            json.get("content").unwrap().as_str().unwrap().to_string()
        }
        _ => parts[0].clone(),
    };
    if parts.len() == 6 {
        let remainder_json: Value = serde_json::from_str(&parts[5]).unwrap();
        TestCase::Ok {
            path: source_path.display().to_string(),
            description: parts[1].clone(),
            source,
            json: parts[4].clone(),
            remainder: remainder_json
                .get("remainder")
                .unwrap()
                .as_str()
                .unwrap()
                .to_string(),
        }
    } else {
        TestCase::Err {
            path: source_path.display().to_string(),
            description: parts[1].clone(),
            source,
        }
    }
}
