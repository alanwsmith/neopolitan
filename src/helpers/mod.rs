use anyhow::Error;
use anyhow::Result;
use std::fs;
use std::path::PathBuf;
use walkdir::WalkDir;

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

pub fn get_test_data(
    source_path: &PathBuf,
) -> Result<(String, String, String, String)> {
    let content = fs::read_to_string(source_path)?;
    let parts: Vec<_> = content
        .split("~~~~~~\n")
        .filter_map(|part| Some(part.trim_end().to_string()))
        .collect();
    if parts.len() == 3 {
        Ok((
            parts[0].clone(),
            parts[1].clone(),
            parts[2].clone(),
            source_path.display().to_string(),
        ))
    } else {
        Err(Error::msg(format!(
            "malformed test file: {}",
            source_path.display()
        )))
    }
}
