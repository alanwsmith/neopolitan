pub mod basic;
pub mod end;
pub mod json;
pub mod raw;
pub mod text_block;

use crate::block::basic::basic_block;
use crate::block::json::json_block;
use crate::block::raw::raw_block;
use crate::block_metadata::parent::BlockParent;
use crate::config::Config;
use crate::span::Span;
use nom::Parser;
use nom::{IResult, branch::alt};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::BTreeMap;

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum JsonData {
    Ok(Value),
    Error(String),
    None,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(tag = "category", rename_all = "lowercase")]
pub enum Block {
    Basic {
        attrs: BTreeMap<String, Vec<Span>>,
        children: Vec<Block>,
        end_block: Option<Box<Block>>,
        flags: Vec<String>,
        kind: String,
    },
    CheckListItem,
    CheckList,
    Csv,
    // TODO: Rename to "Close"
    End {
        attrs: BTreeMap<String, Vec<Span>>,
        children: Vec<Block>,
        flags: Vec<String>,
        kind: String,
    },
    // TODO: Set up Json so that it's
    // top level is an `ok` or `error`
    // based of it it was able to be parsed
    // or not. Though about making it json5
    // but that might be a bridge to far?
    // need to think about that a bit.
    // If you do it, you'd have to figure out
    // how to deal with comments and such
    // in the AST which feels fraught.
    Json {
        attrs: BTreeMap<String, Vec<Span>>,
        data: JsonData,
        end_block: Option<Box<Block>>,
        flags: Vec<String>,
        kind: String,
    },
    List,
    #[serde(rename = "list-item")]
    ListItem,
    #[serde(rename = "numbered-list")]
    NumberedList,
    #[serde(rename = "numbered-list-item")]
    NumberedListItem,
    Raw {
        attrs: BTreeMap<String, Vec<Span>>,
        body: Option<String>,
        end_block: Option<Box<Block>>,
        flags: Vec<String>,
        kind: String,
    },
    #[serde(rename = "text-block")]
    TextBlock {
        spans: Vec<Span>,
    },
}

pub fn block<'a>(
    source: &'a str,
    config: &'a Config,
    parent: &'a BlockParent,
) -> IResult<&'a str, Block> {
    let (source, section) = alt((
        |src| raw_block(src, config, parent),
        |src| json_block(src, config, parent),
        // Make sure to keep basic in the last slot
        |src| basic_block(src, config, parent),
    ))
    .parse(source)?;
    Ok((source, section))
}
