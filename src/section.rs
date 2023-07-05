use crate::block::Block;
use crate::sec_attr::SecAttr;

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
