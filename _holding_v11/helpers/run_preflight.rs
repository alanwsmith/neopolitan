use std::path::PathBuf;
use std::process::Command;
use walkdir::Error;
use walkdir::WalkDir;

pub fn run_preflight() -> Result<(), Error> {
    for entry in WalkDir::new("./site/content").sort_by_file_name() {
        let path = PathBuf::from(entry?.path());
        if let Some(ext) = path.extension() {
            if ext == "py" {
                let path_string = path.display().to_string();
                let args: Vec<&str> = vec![path_string.as_str()];
                let _cmd_output = Command::new("/opt/homebrew/bin/python3")
                    .args(args)
                    .output()
                    .unwrap();
            }
        }
    }
    Ok(())
}
