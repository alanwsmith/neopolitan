pub mod attr;
pub mod basic;
pub mod blocks;
pub mod bound;
pub mod category;
pub mod end;
pub mod flag;
pub mod metadata;
pub mod parent;

use crate::config::Config;
use crate::section::basic::basic_section;
use crate::section::bound::SectionBound;
use crate::section::parent::SectionParent;
use crate::span::Span;
use nom::Parser;
use nom::{IResult, branch::alt};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
// #[serde(untagged)]
// #[serde(tag = "type", content = "content", rename_all = "lowercase")]
#[serde(tag = "category", rename_all = "lowercase")]
pub enum Section {
    Basic {
        attrs: BTreeMap<String, Vec<Span>>,
        bound: SectionBound,
        children: Vec<Section>,
        end_section: Option<Box<Section>>,
        flags: Vec<String>,
        kind: String,
    },
    #[serde(rename = "text-block")]
    CheckListItem,
    CheckList,
    CSS,
    CSV,
    End {
        attrs: BTreeMap<String, Vec<Span>>,
        bound: SectionBound,
        children: Vec<Section>,
        flags: Vec<String>,
        kind: String,
    },
    Html,
    JavaScript,
    Json5,
    List,
    ListItem,
    Olist,
    OlistItem,
    Paragraph {
        spans: Vec<Span>,
    },
    Raw,
}

// #[derive(Debug, Deserialize, PartialEq, Serialize)]
// pub struct Section {
//     pub category: SectionCategory,
//     pub kind: String,
// }

pub fn section<'a>(
    source: &'a str,
    config: &'a Config,
    parent: &'a SectionParent,
    debug: bool,
) -> IResult<&'a str, Section> {
    let (source, section) =
        alt((|src| basic_section(src, config, parent, debug),))
            .parse(source)?;
    Ok((source, section))
}

#[cfg(test)]
mod test {
    // Tests are currently done at the individual section levels
    // or above in the AST
}
