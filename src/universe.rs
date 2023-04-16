use crate::create_env::create_env;
use crate::parse::parse;
use crate::render_template::render_template;
// use crate::universe::universe::*;
// use crate::universe::Universe;
use crate::wrapper::wrapper::*;
use serde::Serialize;
use std::fs;
use std::path::PathBuf;
use walkdir::{DirEntry, Error, WalkDir};

#[derive(Debug, PartialEq, Serialize)]
pub struct Universe {
    pub dest_dir: Option<String>,
    pub pages: Option<Vec<SourceFile>>,
    pub source_dir: Option<String>,
    pub current_source_index: u32,
}

#[derive(Debug, PartialEq, Serialize)]
pub struct SourceFile {
    path: Option<String>,
    source: Option<String>,
    parsed: Option<Wrapper>,
}

impl Universe {
    pub fn load_pages(&mut self) -> Result<(), Error> {
        let walker = WalkDir::new(&self.source_dir.as_ref().unwrap()).into_iter();
        for entry in walker.filter_entry(|e| !is_hidden(e)) {
            let p = entry?.path().to_path_buf();
            match p.extension() {
                Some(v) => {
                    if v == "neo" {
                        let mut sf = SourceFile {
                            path: Some(p.as_os_str().to_str().unwrap().to_string()),
                            source: None,
                            parsed: None,
                        };
                        sf.source =
                            Some(fs::read_to_string(p.as_os_str().to_str().unwrap()).unwrap());
                        sf.parsed = Some(parse(sf.source.as_ref().unwrap().as_str()).unwrap().1);
                        self.pages.as_mut().unwrap().push(sf);
                    }
                }
                None => (),
            }
        }
        Ok(())
    }
}

impl Universe {
    pub fn generate_pages(&self) {
        let env = create_env("./templates");
        for page in self.pages.as_ref().unwrap().iter().enumerate() {
            // self.current_source_index = page.0 as u32;
            let initial_path = PathBuf::from(&page.1.path.as_ref().unwrap());
            let source_path = initial_path
                .strip_prefix(&self.source_dir.as_ref().unwrap())
                .unwrap();
            let mut dest_path = PathBuf::from(&self.dest_dir.as_ref().unwrap());
            dest_path.push(source_path);
            let html_path = dest_path.with_extension("html");
            dbg!(&html_path);
            let output = render_template(self, page.0 as u32, env.clone(), "default.jinja");
            fs::write(html_path, output).unwrap();
        }
    }
}

fn is_hidden(entry: &DirEntry) -> bool {
    entry
        .file_name()
        .to_str()
        .map(|s| s.starts_with("."))
        .unwrap_or(false)
}
