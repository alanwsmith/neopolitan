use crate::snippet::Snippet;

pub mod headline;
pub mod paragraph;

#[derive(Debug, PartialEq)]
pub enum Block {
    Headline { content: Vec<Snippet> },
    Paragraph { content: Vec<Snippet> },
}
