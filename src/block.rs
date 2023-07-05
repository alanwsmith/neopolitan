use crate::snippet::Snippet;

#[derive(Debug, PartialEq)]
pub enum Block {
    Headline { content: Vec<Snippet> },
    Paragraph { content: Vec<Snippet> },
}
