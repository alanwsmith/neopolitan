use crate::snippet::Snippet;

pub mod headline;

#[derive(Debug, PartialEq)]
pub enum Block {
    Headline { content: Vec<Snippet> },
    Paragraph { content: Vec<Snippet> },
}
