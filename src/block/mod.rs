pub mod basic;
pub mod end;
pub mod paragraph;

use crate::block::basic::basic_section;
use crate::block_metadata::bound::BlockBound;
use crate::block_metadata::parent::SectionParent;
use crate::config::Config;
use crate::span::Span;
use nom::Parser;
use nom::{IResult, branch::alt};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
// #[serde(untagged)]
// #[serde(tag = "type", content = "content", rename_all = "lowercase")]
#[serde(tag = "category", rename_all = "lowercase")]
pub enum Block {
    Basic {
        attrs: BTreeMap<String, Vec<Span>>,
        bound: BlockBound,
        children: Vec<Block>,
        end_section: Option<Box<Block>>,
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
        bound: BlockBound,
        children: Vec<Block>,
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

pub fn block<'a>(
    source: &'a str,
    config: &'a Config,
    parent: &'a SectionParent,
    debug: bool,
) -> IResult<&'a str, Block> {
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
