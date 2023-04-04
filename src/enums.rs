// use std::collections::HashMap;

#[derive(Debug, PartialEq)]
pub enum SectionAttribute {
    Attribute {
        key: Option<String>,
        value: Option<String>,
    },
}

#[derive(Debug, PartialEq)]
pub enum Wrapper {
    Page { children: Option<Vec<Section>> },
}

#[derive(Debug, PartialEq)]
pub enum Section {
    Title {
        // has to be a vec becosre order matters
        // for the code sections
        attributes: Option<Vec<SectionAttribute>>,
        children: Option<Vec<Block>>,
    },
    Paragraphs {
        children: Option<Vec<Block>>,
    },
}

#[derive(Debug, PartialEq)]
pub enum Block {
    P { children: Option<Vec<Content>> },
}

#[derive(Debug, PartialEq)]
pub enum Content {
    Link { text: String, url: String },
    Text { text: String },
    Space,
}
