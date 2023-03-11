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

// pub fn get_sections(data: &str) -> IResult<&str, Vec<Section>> {
//     Ok(("", vec![]))
// }

pub fn parse(source: &str) -> Page {
    let page = Page {
        attributes: HashMap::new(),
        children: vec![get_title(source)],
    };
    page
}
