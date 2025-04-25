use crate::{
    section::Section, section_attribute::SectionAttribute,
    section_bound::SectionBound, span::Span,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub enum SectionCategory {
    Basic {
        attrs: Vec<SectionAttribute>,
        bound: SectionBound,
        children: Vec<Section>,
        end_section: Option<Box<Section>>,
        flags: Vec<String>,
        source_head: String,
        source_body: Option<String>,
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
