use serde::Serialize;

#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "type")]
pub enum SnippetAttribute {
    Attribute {
        key: Option<String>,
        value: Option<String>,
    },
}