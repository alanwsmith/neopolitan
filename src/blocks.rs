use crate::tags::Tag;

pub mod headline;
pub mod paragraph;

#[derive(Debug, PartialEq)]
pub enum Block {
    Headline { tags: Vec<Tag> },
    Paragraph { tags: Vec<Tag> },
}
