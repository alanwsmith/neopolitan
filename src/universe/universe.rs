#![allow(warnings)]
use crate::parse::parse::parse;
use crate::source_file::source_file::SourceFile;
use eyre::Error as Error2;
use fs_extra::copy_items;
use fs_extra::dir;
use minijinja::context;
use minijinja::Environment;
use std::fs;
use std::fs::create_dir_all;
use std::path::PathBuf;
use walkdir::{DirEntry, Error, WalkDir};

#[derive(Debug)]
pub struct Universe<'a> {
    pub assets_dir: Option<PathBuf>,
    pub dest_dir: Option<PathBuf>,
    pub env: Option<Environment<'a>>,
    pub source_files: Vec<SourceFile>,
    pub source_dir: Option<PathBuf>,
}

impl Universe<'_> {
    pub fn new() -> Universe<'static> {
        Universe {
            assets_dir: None,
            dest_dir: None,
            env: None,
            source_dir: None,
            source_files: vec![],
        }
    }
}

impl Universe<'_> {
    pub fn load_files(&mut self) -> Result<(), Error> {
        let mut count: i32 = 0;
        for entry in WalkDir::new(&self.source_dir.as_ref().unwrap()).into_iter() {
            let p = entry?.path().to_path_buf();
            if let Some(ext) = p.extension() {
                if ext == "neo" {
                    let mut sf = SourceFile::new();
                    sf.input_path = Some(p.clone());
                    dbg!(&sf.input_path);
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
            count += 1;
            println!("{}", count);
        }
        Ok(())
    }
}

impl Universe<'_> {
    pub fn output_files(&self) {
        let mut count: i32 = 0;
        for output_file in self.source_files.iter() {
            let mut output_path = PathBuf::from(self.dest_dir.as_ref().unwrap());
            let sub_path = &output_file
                .input_path
                .as_ref()
                .unwrap()
                .strip_prefix(&self.source_dir.as_ref().unwrap());
            output_path.push(sub_path.as_ref().unwrap());
            output_path.set_extension("html");
            create_dir_all(output_path.parent().unwrap());
            let wrapper = self
                .env
                .as_ref()
                .unwrap()
                .get_template("default.j2")
                .unwrap();
            let out = wrapper
                .render(context!(
                content =>
                 output_file.output(self).unwrap()
                    ))
                .unwrap()
                .to_string();
            //dbg!(&output_path);
            fs::write(output_path, out).unwrap();
            count += 1;
            println!("{}", count);
        }
    }
}

impl Universe<'_> {
    pub fn load_assets_didnot_work(&self) -> Result<u64, Error2> {
        let options = dir::CopyOptions {
            buffer_size: 64000,
            content_only: true,
            copy_inside: false,
            depth: 0,
            overwrite: true,
            skip_exist: true,
        };
        let mut from_paths = Vec::new();
        from_paths.push(self.assets_dir.as_ref().unwrap());
        copy_items(&from_paths, self.dest_dir.as_ref().unwrap(), &options)?;
        Ok(0)
    }
}
