#[derive(Debug, PartialEq)]
pub enum Block {
    Text { snippets: Option<Vec<Snippet>> },
}

#[derive(Debug, PartialEq)]
pub enum Snippet {
    Plain { text: Option<String> },
}
