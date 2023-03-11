use serde::{Deserialize, Serialize};
use std::collections::HashMap;
// use serde_json::Result;

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Page {
    attributes: HashMap<String, String>,
    children: Vec<Section>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
enum Section {
    Title {
        attributes: HashMap<String, String>,
        children: Vec<Content>,
    },
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
enum Content {
    Text { value: String },
}

pub fn parse(_source: String) -> Page {
    let page = Page {
        attributes: HashMap::new(),
        children: vec![Section::Title {
            attributes: HashMap::new(),
            children: vec![Content::Text {
                value: "This is a title".to_string(),
            }],
        }],
    };
    page
}
