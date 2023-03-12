use crate::content::Content;
use crate::get_blocks::*;
use crate::page::Page;
use crate::section::Section;
use std::collections::HashMap;

pub fn parse_dev(source: &str) -> Page {
    let page = Page {
        attributes: HashMap::new(),
        children: vec![],
    };
    dbg!(&page);
    // Get the raw blocks to process
    let blocks = get_blocks(source).unwrap().1;
    dbg!(blocks);
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
