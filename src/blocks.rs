use crate::tags::Tag;
use serde::Serialize;

pub mod headline;
pub mod paragraph;

#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "type")]
pub enum Block {
    Headline { tags: Vec<Tag> },
    Paragraph { tags: Vec<Tag> },
}
