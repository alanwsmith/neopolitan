#![allow(warnings)]
use crate::parse::parse::parse;
use crate::source_file::source_file::SourceFile;
use miette::Result;
use minijinja::context;
use minijinja::Environment;
use regex::Regex;
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use urlencoding::encode;
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
    // GOAL: do everything required for setting output variables here
    pub fn load_raw_data(&mut self) -> Result<(), Error> {
        println!("Loading raw data");
        for entry in WalkDir::new(&self.content_dir.as_ref().unwrap()).into_iter() {
            let p = entry?.path().to_path_buf();
            println!("Input: {}", &p.display());
            if let Some(ext) = p.extension() {
                if ext == "neo" {
                    let mut sf = SourceFile::new();
                    let mut initial_string = fs::read_to_string(&p.to_str().unwrap()).unwrap();
                    initial_string.push_str("\n");
                    sf.raw = Some(initial_string);
                    let parsed_data = parse(sf.raw.as_ref().unwrap().as_str());
                    sf.raw_path = Some(
                        p.strip_prefix(&self.content_dir.as_ref().unwrap())
                            .unwrap()
                            .to_path_buf(),
                    );

                    // let raw = fs::read_to_string(&p.to_str().unwrap()).unwrap();
                    // sf.slug_dir = Some(PathBuf::from(&p));
                    // sf.slug_dir = Some(
                    //     sf.slug_dir
                    //         .as_ref()
                    //         .unwrap()
                    //         .strip_prefix(&self.content_dir.as_ref().unwrap())
                    //         .unwrap()
                    //         .to_path_buf(),
                    // );
                    // dbg!(&sf.slug_dir);

                    match parsed_data {
                        Err(_) => {}
                        Ok(data) => {
                            // //  get teh slug path
                            // let file_stem = &p.file_stem().unwrap();
                            // if file_stem.to_str().unwrap() == "index" {
                            //     sf.slug_dir = Some(PathBuf::from(
                            //         p.strip_prefix(&self.content_dir.as_ref().unwrap())
                            //             .unwrap()
                            //             .parent()
                            //             .unwrap()
                            //             .to_path_buf()
                            //             .display()
                            //             .to_string()
                            //             .to_lowercase(),
                            //     ));
                            // } else {
                            //     sf.slug_dir = Some(
                            //         p.strip_prefix(&self.content_dir.as_ref().unwrap())
                            //             .unwrap()
                            //             .with_extension(""),
                            //     );
                            // }

                            sf.parsed = data.1;
                            if sf.status() == Some("published".to_string()) {
                                self.content_files.insert(p.canonicalize().unwrap(), sf);
                            } else if sf.status() == Some("draft".to_string()) {
                                self.content_files.insert(p.canonicalize().unwrap(), sf);
                            } else if sf.status() == Some("scratch".to_string()) {
                                self.content_files.insert(p.canonicalize().unwrap(), sf);
                            }
                        }
                    }
                }
            }
        }
        Ok(())
    }
}

impl Universe<'_> {
    pub fn scrub_url_path(source: String) -> String {
        let re = Regex::new(r"\s+").unwrap();
        re.replace_all(&source, "-").to_lowercase()
    }
}

// impl Universe<'_> {
//     pub fn output_index_file(&self) {
//         println!("Outputting Index File");
//         let mut links = vec![];
//         for (_source_path, source_file) in self.content_files.iter() {
//             let mut url = String::from("/posts/");
//             url.push_str(source_file.slug_dir().as_ref().unwrap().to_str().unwrap());
//             url.push_str("/index.html");
//             links.push(url);
//             // dbg!(url);
//             // self.output_file(source_path.to_path_buf());
//             // dbg!(source_file.output_path.as_ref());
//             //dbg!(source_file.slug_dir().as_ref().unwrap());
//         }
//         // if let Some(_) = source_file.output(self) {
//         let wrapper = self.env.as_ref().unwrap().get_template("index.j2").unwrap();
//         let tmp_index_file = "/Users/alan/workshop/alanwsmith.com/site/index.html";
//         let out = wrapper
//             .render(context!(
//             content =>
//             links
//                 ))
//             .unwrap()
//             .to_string();
//          fs::write(tmp_index_file, out).unwrap();
//         // }
//     }
// }

// NOTE: The source paths are the file paths on disk.
// They are used here as keys so individual files can be
// updated.

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
        println!("Output finished");
        println!("Total Files: {}", counter);
    }
}

impl Universe<'_> {
    pub fn output_file(&self, path: PathBuf) {
        // let source_file = self.content_files.get(&path);
        // dbg!(path.display());
        if let Some(source_file) = self.content_files.get(&path) {
            dbg!(&source_file.raw_path.as_ref().unwrap());

            // let mut output_path = self.output_root.clone().unwrap();
            // output_path.push(&source_file.raw_path.as_ref().unwrap());
            // dbg!(output_path);

            // let mut output_file = output_dir.clone();
            // fs::create_dir_all(output_dir);
            // output_file.push("index.html");
            // dbg!(&output_file);
            // let mut template_file = source_file.file_type().unwrap().1.unwrap();
            // template_file.push_str(".j2");
            // if let Some(_) = source_file.output(self) {
            //     let wrapper = self
            //         .env
            //         .as_ref()
            //         .unwrap()
            //         .get_template(&template_file)
            //         .unwrap();
            //     let out = wrapper
            //         .render(context!(
            //         content =>
            //          source_file.output(self).unwrap()
            //             ))
            //         .unwrap()
            //         .to_string();
            //     fs::write(output_file, out).unwrap();
            // }

        }
    }
}
