#[derive(Debug, PartialEq)]
pub enum Wrapper {
    Page { children: Vec<Section> },
}

#[derive(Debug, PartialEq)]
pub enum Section {
    Title { children: Vec<Block> },
    Paragraphs { children: Vec<Block> },
}

#[derive(Debug, PartialEq)]
pub enum Block {
    P { children: Vec<Content> },
}

#[derive(Debug, PartialEq)]
pub enum Content {
    Link { text: String, url: String },
    Text { text: String },
    Space,
}
