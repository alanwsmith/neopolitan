#[derive(Debug, PartialEq)]
pub enum Attribute {
    Basic {
        key: Option<String>,
        value: Option<String>,
    },
}
