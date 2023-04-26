use serde::Serialize;

#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "type")]
pub enum ListItem {
    Basic { children: Option<Vec<String>> },
}
