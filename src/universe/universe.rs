use crate::parse::parse::parse;
use crate::source_file::source_file::SourceFile;
use miette::Result;
use minijinja::context;
use minijinja::Environment;
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use walkdir::Error;
use walkdir::WalkDir;

#[derive(Debug, Clone)]
pub struct Universe<'a> {
    pub assets_dir: Option<PathBuf>,
    pub content_dir: Option<PathBuf>,
    pub content_files: HashMap<PathBuf, SourceFile>,
    pub env: Option<Environment<'a>>,
    pub output_root: Option<PathBuf>,
}

impl Universe<'_> {
    pub fn new() -> Universe<'static> {
        Universe {
            assets_dir: None,
            content_dir: None,
            content_files: HashMap::new(),
            env: None,
            output_root: None,
        }
    }
}

impl Universe<'_> {
    pub fn find_files(&mut self) -> Result<(), Error> {
        println!("Finding files");
        for entry in WalkDir::new(&self.content_dir.as_ref().unwrap()).into_iter() {
            let p = entry?.path().to_path_buf();
            if let Some(ext) = p.extension() {
                if ext == "neo" {
                    self.content_files
                        .insert(p.canonicalize().unwrap(), SourceFile::new());
                }
            }
        }
        Ok(())
    }
}

impl Universe<'_> {
    pub fn load_raw_data(&mut self) {
        for (path, sf) in self.content_files.iter_mut() {
            sf.raw = Some(fs::read_to_string(path.as_os_str().to_str().unwrap()).unwrap());
            let parsed_data = parse(sf.raw.as_ref().unwrap().as_str());
            match parsed_data {
                Err(_) => sf.parsed = None,
                Ok(data) => {
                    sf.parsed = data.1;
                }
            }
        }
    }
}

impl Universe<'_> {
    pub fn output_files(&self) {
        println!("Outputting files");
        let mut counter: u32 = 0;
        for (source_path, _source_file) in self.content_files.iter() {
            self.output_file(source_path.to_path_buf());
            counter += 1;
            if counter % 100 == 0 {
                println!("Count: {}", counter);
            }
        }
        println!("Count: {}", counter);
    }
}

impl Universe<'_> {
    pub fn output_file(&self, path: PathBuf) {
        // let source_file = self.content_files.get(&path);
        println!("{}", path.display());
        if let Some(source_file) = self.content_files.get(&path) {
            let output_path = self.get_output_path(path);
            // dbg!(output_path);
            if let Some(_) = source_file.output(self) {
                let wrapper = self
                    .env
                    .as_ref()
                    .unwrap()
                    .get_template("default.j2")
                    .unwrap();
                let out = wrapper
                    .render(context!(
                    content =>
                     source_file.output(self).unwrap()
                        ))
                    .unwrap()
                    .to_string();
                fs::write(output_path, out).unwrap();
            }
        }
    }
}

impl Universe<'_> {
    pub fn get_output_path(&self, path: PathBuf) -> PathBuf {
        let mut output_path = PathBuf::from(self.output_root.as_ref().unwrap());
        let sub_path = path.strip_prefix(&self.content_dir.as_ref().unwrap());
        output_path.push(sub_path.as_ref().unwrap());
        output_path.set_extension("html");
        output_path
    }
}
