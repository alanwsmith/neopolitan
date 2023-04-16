#![allow(warnings)]
use minijinja::context;
use minijinja::AutoEscape;
use minijinja::Environment;
use minijinja::Source;
use neopolitan::block::block::Block::RawContent;
use neopolitan::create_env::create_env;
use neopolitan::parse::parse;
use neopolitan::render_template::render_template;
use neopolitan::section::attributes_for_section::SectionAttribute::Attribute;
use neopolitan::section::section::*;
use neopolitan::wrapper::wrapper::*;
use std::fs;
use std::fs::create_dir_all;
use std::path::PathBuf;
use walkdir::{DirEntry, Error, WalkDir};

fn main() {
    do_copy("./content", "./site").unwrap();
    println!("PROCESS COMPLETE");
}

fn is_hidden(entry: &DirEntry) -> bool {
    entry
        .file_name()
        .to_str()
        .map(|s| s.starts_with("."))
        .unwrap_or(false)
}

fn do_copy(source_dir: &str, dest_dir: &str) -> Result<(), Error> {
    let env = create_env("./templates");
    let walker = WalkDir::new(source_dir).into_iter();
    for entry in walker.filter_entry(|e| !is_hidden(e)) {
        let p = entry?.path().to_path_buf();
        let sub_path = &p.strip_prefix(source_dir);
        let mut dest_path = PathBuf::from(dest_dir);
        dest_path.push(sub_path.as_ref().unwrap());
        if p.is_dir() {
            if !dest_path.exists() {
                create_dir_all(dest_path).unwrap();
            }
        } else {
            if p.extension().unwrap() == "neo" {
                let html_path = dest_path.with_extension("html");
                println!("File: {}", p.as_os_str().to_str().unwrap());
                let source = fs::read_to_string(p.as_os_str().to_str().unwrap()).unwrap();
                let mut payload = parse(source.as_str()).unwrap().1;
                let mut source_type = get_type(&payload);
                source_type.push_str(".jinja");
                let should_publish = should_publish(&payload);
                if should_publish == "y".to_string() {
                    println!("- Rendering");
                    let output = render_template(payload, env.clone(), source_type.as_str());
                    fs::write(html_path, output).unwrap();
                } else {
                    println!("- Skipping");
                }
            }
            // NOTE: for now, just always copy the source files over to
            // easy examples. That will be put behind a config flag
            // with the default being off.
            fs::copy(p, dest_path);
        }
    }
    Ok(())
}

fn get_type(payload: &Wrapper) -> String {
    match &payload {
        Wrapper::Page { children } => {
            for section in children.as_ref().unwrap() {
                match section {
                    Section::AttributesSection { attributes, .. } => {
                        for section_attribute in attributes.as_ref().unwrap() {
                            match section_attribute {
                                Attribute { key, value } => {
                                    if key.as_ref().unwrap() == "type" {
                                        return value.as_ref().unwrap().trim().to_string();
                                    }
                                }
                                _ => {}
                            }
                        }
                    }
                    _ => {}
                }
            }
        }
    }
    "default".to_string()
}

fn should_publish(payload: &Wrapper) -> String {
    match &payload {
        Wrapper::Page { children } => {
            for section in children.as_ref().unwrap() {
                match section {
                    Section::AttributesSection { attributes, .. } => {
                        for section_attribute in attributes.as_ref().unwrap() {
                            match section_attribute {
                                Attribute { key, value } => {
                                    if key.as_ref().unwrap() == "publish" {
                                        return value.as_ref().unwrap().trim().to_string();
                                    }
                                }
                                _ => {}
                            }
                        }
                    }
                    _ => {}
                }
            }
        }
    }
    "default".to_string()
}
