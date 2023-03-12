use crate::content::Content;
use crate::page::Page;
use crate::section::Section;
use std::collections::HashMap;

pub fn parse_dev(_source: &str) -> Page {
    Page {
        attributes: HashMap::new(),
        children: vec![Section::TITLE {
            attributes: HashMap::new(),
            children: vec![Content::PlainText {
                value: "This Is A Title".to_string(),
            }],
        }],
    }
}
