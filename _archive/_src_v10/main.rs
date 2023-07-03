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
use neopolitan::universe::Universe;
use neopolitan::wrapper::wrapper::*;
use std::fs;
use std::fs::create_dir_all;
use std::path::PathBuf;
use walkdir::{DirEntry, Error, WalkDir};

fn main() {
    let mut u = Universe {
        assets_dir: Some(String::from("./assets")),
        dest_dir: Some(String::from("./sites/default")),
        // current_source_index: 0,
        pages: Some(vec![]),
        source_dir: Some(String::from("./content")),
    };
    u.load_pages();
    u.copy_assets();
    u.generate_pages();
    println!("------------ PROCESS COMPLETE --------------");
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
    "n".to_string()
}
