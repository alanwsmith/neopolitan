use crate::block::block::*;
use crate::section::section_attributes::SectionAttribute;

#[derive(Debug, PartialEq)]
pub enum Section {
    Title {
        // has to be a vec becosre order matters
        // for the code sections
        attributes: Option<Vec<SectionAttribute>>,
        children: Option<Vec<Block>>,
    },
    Paragraphs {
        attributes: Option<Vec<SectionAttribute>>,
        children: Option<Vec<Block>>,
    },
}
