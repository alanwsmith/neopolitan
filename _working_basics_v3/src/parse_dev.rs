use crate::get_attributes::get_attributes;
use crate::get_blocks::*;
use crate::get_categories::get_categories;
use crate::get_list::get_list;
use crate::get_ordered_list::get_ordered_list;
use crate::get_paragraphs::*;
use crate::page::Page;
use crate::section::Section;
use std::collections::HashMap;

pub fn parse_dev(source: &str) -> Page {
    let mut page = Page {
        attributes: HashMap::new(),
        children: vec![],
        categories: vec![],
    };
    let blocks = get_blocks(source).unwrap().1;
    for block in blocks {
        match block {
            Block::UNORDERED_LIST { source } => {
                let (_, list) = get_list(source.as_str()).unwrap();
                page.children.push(list);
            }
            Block::TITLE { source } => {
                page.children.push(Section::TITLE {
                    attributes: HashMap::new(),
                    children: vec![Section::PLAINTEXT {
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
            Block::CATEGORIES { source } => {
                let (_, categories) = get_categories(source.as_str()).unwrap();
                for category in categories {
                    page.categories.push(category);
                }
            }
            Block::ATTRIBUTES { source } => {
                let (_, attributes) = get_attributes(source.as_str()).unwrap();
                for (key, value) in attributes.iter() {
                    page.attributes.insert(key.to_string(), value.to_string());
                }
            }
            Block::ORDERED_LIST { source } => {
                dbg!(&source);
                let (_, list) = get_ordered_list(source.as_str()).unwrap();
                page.children.push(list);
            }
            Block::PLACEHOLDER {} => {}
        }
    }
    // dbg!(&page);

    // let page = Page {
    //     attributes: HashMap::new(),
    //     children: vec![Section::ORDERED_LIST {
    //         attributes: HashMap::new(),
    //         children: vec![Section::ORDERED_LIST_ITEM {
    //             attributes: HashMap::new(),
    //             children: vec![Section::P {
    //                 attributes: HashMap::new(),
    //                 children: vec![Section::PLAINTEXT {
    //                     value: "echo foxtrot".to_string(),
    //                 }],
    //             }],
    //         }],
    //     }],
    //     categories: vec![],
    // };

    // let page = Page {
    //     attributes: HashMap::new(),
    //     children: vec![],
    //     categories: vec!["Rust".to_string(), "Test".to_string()],
    // };

    page
}