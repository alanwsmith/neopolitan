use crate::snippets::Snippet;

pub mod headline;
pub mod paragraph;

#[derive(Debug, PartialEq)]
pub enum Block {
    Headline { snippets: Vec<Snippet> },
    Paragraph { snippets: Vec<Snippet> },
}
