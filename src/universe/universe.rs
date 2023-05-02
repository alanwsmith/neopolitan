#![allow(warnings)]

use crate::parse::parse::parse;
use crate::source_file::source_file::SourceFile;
use crate::universe::create_env::create_env;
// use core::fmt::Error;
use eyre::Error as Error2;
use fs_extra::copy_items;
use fs_extra::dir;
use minijinja::context;
use minijinja::Environment;
// use neopolitan::helpers::load_assets::load_assets;
// use neopolitan::universe::create_env::create_env;
// use neopolitan::universe::universe::Universe;
// use core::fmt::Error;
use miette::Result;
use std::collections::HashMap;
use std::fs;
use std::fs::create_dir_all;
use std::path::PathBuf;
use std::time::Duration;
use walkdir::{DirEntry, Error, WalkDir};
use watchexec::action::Action;
use watchexec::action::Outcome;
use watchexec::config::InitConfig;
use watchexec::config::RuntimeConfig;
use watchexec::Watchexec;
use watchexec_signals::Signal;

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
                Err(e) => sf.parsed = None,
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
        for (source_path, source_file) in self.content_files.iter() {
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

// impl Universe<'_> {
//     pub fn watch_files(&self) -{
//         let mut output_path = PathBuf::from(self.output_root.as_ref().unwrap());
//         let sub_path = path.strip_prefix(&self.content_dir.as_ref().unwrap());
//         output_path.push(sub_path.as_ref().unwrap());
//         output_path.set_extension("html");
//         output_path
//     }
// }

// impl Universe<'_> {
//     pub fn load_files(&mut self) -> Result<(), Error> {
//         let mut count: i32 = 0;
//         for entry in WalkDir::new(&self.source_dir.as_ref().unwrap()).into_iter() {
//             let p = entry?.path().to_path_buf();
//             if let Some(ext) = p.extension() {
//                 if ext == "neo" {
//                     let mut sf = SourceFile::new();
//                     sf.input_path = Some(p.clone());
//                     dbg!(&sf.input_path);
//                     sf.raw = Some(
//                         fs::read_to_string(
//                             sf.input_path
//                                 .as_ref()
//                                 .unwrap()
//                                 .as_os_str()
//                                 .to_str()
//                                 .unwrap(),
//                         )
//                         .unwrap(),
//                     );
//                     let parsed_data = parse(sf.raw.as_ref().unwrap().as_str());
//                     match parsed_data {
//                         Err(e) => {}
//                         Ok(data) => {
//                             sf.parsed = data.1;
//                             &self.source_files.push(sf);
//                         }
//                     }
//                     // sf.parsed = parse(sf.raw.as_ref().unwrap().as_str()).unwrap().1;
//                     // &self.source_files.push(sf);
//                 }
//             }
//             count += 1;
//             println!("{}", count);
//         }
//         Ok(())
//     }
// }

// impl Universe<'_> {
//     pub fn output_files(&self) {
//         let mut count: i32 = 0;
//         for output_file in self.source_files.iter() {
//             let mut output_path = PathBuf::from(self.dest_dir.as_ref().unwrap());
//             let sub_path = &output_file
//                 .input_path
//                 .as_ref()
//                 .unwrap()
//                 .strip_prefix(&self.source_dir.as_ref().unwrap());
//             output_path.push(sub_path.as_ref().unwrap());
//             output_path.set_extension("html");
//             create_dir_all(output_path.parent().unwrap());
//             let wrapper = self
//                 .env
//                 .as_ref()
//                 .unwrap()
//                 .get_template("default.j2")
//                 .unwrap();
//             let out = wrapper
//                 .render(context!(
//                 content =>
//                  output_file.output(self).unwrap()
//                     ))
//                 .unwrap()
//                 .to_string();
//             //dbg!(&output_path);
//             fs::write(output_path, out).unwrap();
//             count += 1;
//             println!("{}", count);
//         }
//     }
// }

// impl Universe<'_> {
//     pub fn load_assets_didnot_work(&self) -> Result<u64, Error2> {
//         let options = dir::CopyOptions {
//             buffer_size: 64000,
//             content_only: true,
//             copy_inside: false,
//             depth: 0,
//             overwrite: true,
//             skip_exist: true,
//         };
//         let mut from_paths = Vec::new();
//         from_paths.push(self.assets_dir.as_ref().unwrap());
//         copy_items(&from_paths, self.dest_dir.as_ref().unwrap(), &options)?;
//         Ok(0)
//     }
// }

// impl Universe<'_> {
//     #[tokio::main]
//     pub async fn watch_files(&self) -> Result<(), Error> {
//         let init = InitConfig::default();
//         let mut runtime = RuntimeConfig::default();
//         runtime.pathset([self.source_dir.as_ref().unwrap()]);
//         runtime.on_action(move |action: Action| async move {
//             let mut stop: bool = false;
//             let mut paths: Vec<PathBuf> = vec![];
//             action.events.iter().for_each(|event| {
//                 event.signals().for_each(|sig| match sig {
//                     Signal::Interrupt => {
//                         println!("Caught Interrupt: Stopping");
//                         stop = true;
//                     }
//                     _ => {}
//                 });
//                 event
//                     .paths()
//                     .for_each(|path| paths.push(path.0.to_path_buf()));
//             });
//             if stop {
//                 action.outcome(Outcome::Exit);
//             }
//             paths.dedup();
//             for path in paths.iter() {
//                 // self.update_file(path.to_path_buf());
//                 dbg!(path);
//             }
//             //paths
//             //j.iter()
//             // .for_each(|path| &self.update_file(path.clone()));
//             // .for_each(|path| self.update_file(path));
//             Ok::<(), Error>(())
//         });
//         let we = Watchexec::new(init, runtime).unwrap();
//         we.main().await.unwrap().unwrap();
//         Ok(())
//     }
// }
