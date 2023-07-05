use crate::tags::Tag;

pub mod headline;
pub mod paragraph;

#[derive(Debug, PartialEq)]
pub enum Block {
    Headline { snippets: Vec<Tag> },
    Paragraph { snippets: Vec<Tag> },
}
