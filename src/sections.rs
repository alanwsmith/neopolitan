use crate::blocks::Block;
use crate::section_attrs::SecAttr;

pub mod title;

#[derive(Debug, PartialEq)]
pub enum Section {
    Title {
        attrs: Vec<SecAttr>,
        headline: Block,
        paragraphs: Vec<Block>,
    },
    None,
}
