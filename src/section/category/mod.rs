use crate::{section::Section, section::bound::SectionBound, span::Span};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
// #[serde(untagged)]
#[serde(tag = "type", content = "content", rename_all = "lowercase")]
pub enum SectionCategory {
    Basic {
        attrs: BTreeMap<String, Vec<Vec<Span>>>,
        bound: SectionBound,
        children: Vec<Section>,
        end_section: Option<Box<Section>>,
        flags: Vec<String>,
    },
    Block {
        spans: Vec<Span>,
    },
    CheckListItem,
    CheckList,
    CSS,
    CSV,
    End,
    Html,
    JavaScript,
    Json5,
    List,
    ListItem,
    Olist,
    OlistItem,
    Raw,
}
