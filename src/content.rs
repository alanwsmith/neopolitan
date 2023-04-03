#[derive(Debug, PartialEq)]
pub enum Content {
    Text { value: Option<String> },
}
