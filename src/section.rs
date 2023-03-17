use crate::chunk::Chunk;

#[derive(Debug, PartialEq)]
pub enum Section {
    TITLE { children: Vec<Chunk> },
}
