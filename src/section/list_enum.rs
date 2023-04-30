use serde::Serialize;

#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(tag = "type")]
pub enum ListItem {
    Basic { children: Option<Vec<String>> },
}
