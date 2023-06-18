use crate::parse::parse::parse;
use crate::source_file::source_file::SourceFile;
use crate::universe::universe::Universe;
use miette::Result;
use std::fs;
use walkdir::Error;
use walkdir::WalkDir;

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
