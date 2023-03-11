#![allow(unused_imports)]
use nom::IResult;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
// use serde_json::Result;

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Page {
    attributes: HashMap<String, String>,
    children: Vec<Section>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub enum Section {
    Title {
        attributes: HashMap<String, String>,
        children: Vec<Content>,
    },
    H2 {
        attributes: HashMap<String, String>,
        children: Vec<Content>,
    },
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub enum Content {
    Text { value: String },
}

pub fn get_title(source: &str) -> Section {
    Section::Title {
        attributes: HashMap::new(),
        children: vec![get_text(source)],
    }
}

pub fn get_text(_source: &str) -> Content {
    Content::Text {
        value: "This is a title".to_string(),
    }
}

pub fn get_sections(source: &str) -> IResult<&str, Vec<Section>> {
    Ok(("", vec![get_title(source)]))
}

pub fn parse(source: &str) -> Page {
    let page = Page {
        attributes: HashMap::new(),
        children: get_sections(source).unwrap().1,
    };
    page
}

pub fn get_text_dev(_source: &str) -> Content {
    Content::Text {
        value: "This is an H2".to_string(),
    }
}

pub fn get_h2(source: &str) -> Section {
    Section::H2 {
        attributes: HashMap::new(),
        children: vec![get_text_dev(source)],
    }
}

pub fn get_sections_dev(source: &str) -> IResult<&str, Vec<Section>> {
    Ok(("", vec![get_title(source), get_h2(source)]))
}

pub fn parse_dev(source: &str) -> Page {
    let page = Page {
        attributes: HashMap::new(),
        children: get_sections_dev(source).unwrap().1,
    };
    page
}
