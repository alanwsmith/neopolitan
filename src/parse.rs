use crate::content::Content;
use crate::get_blocks::*;
use crate::get_paragraphs::*;
use crate::page::Page;
use crate::section::Section;
use std::collections::HashMap;

pub fn parse(source: &str) -> Page {
    let mut page = Page {
        attributes: HashMap::new(),
        children: vec![],
    };
    let blocks = get_blocks(source).unwrap().1;
    for block in blocks {
        match block {
            Block::TITLE { source } => {
                page.children.push(Section::TITLE {
                    attributes: HashMap::new(),
                    children: vec![Content::PlainText {
                        value: source.to_string(),
                    }],
                });
            }
            Block::P { source } => {
                // dbg!(&source);
                let paragraphs: Vec<Section> = get_paragraphs(source.as_str()).unwrap().1;
                // dbg!(paragraphs);

                for paragraph in paragraphs {
                    page.children.push(paragraph);
                }
            }
        }
    }
    dbg!(&page);
    page
}
