#![allow(warnings)]
use crate::parse::parse::parse;
use crate::source_file::source_file::SourceFile;
use minijinja::Environment;
use std::fs;
use std::path::PathBuf;
use walkdir::{DirEntry, Error, WalkDir};

#[derive(Debug)]
pub struct Universe<'a> {
    pub dest_dir: Option<PathBuf>,
    pub env: Option<Environment<'a>>,
    pub source_files: Vec<SourceFile>,
    pub source_dir: Option<PathBuf>,
}

impl Universe<'_> {
    pub fn new() -> Universe<'static> {
        Universe {
            dest_dir: None,
            env: None,
            source_dir: None,
            source_files: vec![],
        }
    }
}

impl Universe<'_> {
    pub fn load_files(&mut self) -> Result<(), Error> {
        for entry in WalkDir::new(&self.source_dir.as_ref().unwrap()).into_iter() {
            let p = entry?.path().to_path_buf();
            if let Some(ext) = p.extension() {
                if ext == "neo" {
                    let mut sf = SourceFile::new();
                    sf.input_path = Some(p.clone());
                    sf.raw = Some(
                        fs::read_to_string(
                            sf.input_path
                                .as_ref()
                                .unwrap()
                                .as_os_str()
                                .to_str()
                                .unwrap(),
                        )
                        .unwrap(),
                    );
                    sf.parsed = parse(sf.raw.as_ref().unwrap().as_str()).unwrap().1;
                    &self.source_files.push(sf);
                }
            }
        }
        Ok(())
    }
}

impl Universe<'_> {
    pub fn output_files(&self) {
        for output_file in self.source_files.iter() {
            let mut output_path = PathBuf::from(self.dest_dir.as_ref().unwrap());
            let sub_path = &output_file
                .input_path
                .as_ref()
                .unwrap()
                .strip_prefix(&self.source_dir.as_ref().unwrap());
            output_path.push(sub_path.as_ref().unwrap());
            output_path.set_extension("html");
            dbg!(&output_path);
            fs::write(output_path, output_file.output(self).unwrap()).unwrap();
        }
    }
}
