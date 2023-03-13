#![allow(unused_variables)]

use crate::content::Content;
use crate::get_attributes::get_attributes;
use crate::get_blocks::*;
use crate::get_paragraphs::*;
use crate::page::Page;
use crate::section::Section;
use std::collections::HashMap;

pub fn parse(source: &str) -> Page {
    let mut page = Page {
        attributes: HashMap::new(),
        children: vec![],
        categories: vec![],
    };
    let blocks = get_blocks(source).unwrap().1;
    for block in blocks {
        match block {
            Block::TITLE { source } => {
                page.children.push(Section::TITLE {
                    attributes: HashMap::new(),
                    children: vec![Content::PLAINTEXT {
                        value: source.to_string(),
                    }],
                });
            }
            Block::P { source } => {
                let (_, paragraphs) = get_paragraphs(source.as_str()).unwrap();
                for paragraph in paragraphs {
                    page.children.push(paragraph);
                }
            }
            Block::BLURB { source } => {
                page.attributes
                    .insert("blurb".to_string(), source.to_string());
            }
            Block::CATEGORIES { source } => {}
            Block::ATTRIBUTES { source } => {
                let (_, attributes) = get_attributes(source.as_str()).unwrap();

                // dbg!(&attributes);

                for (key, value) in attributes.iter() {
                    // dbg!(&key);
                    // dbg!(&value);
                    page.attributes.insert(key.to_string(), value.to_string());
                }
            }
        }
    }
    // dbg!(&page);

    // let mut page = Page {
    //     attributes: HashMap::new(),
    //     children: vec![],
    // };
    // page.attributes
    //     .insert("date".to_string(), "2023-03-03 04:05:06".to_string());

    page
}
