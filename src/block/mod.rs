pub mod basic;
pub mod end;
pub mod raw;
pub mod text;

use crate::block::basic::basic_block;
use crate::block::raw::raw_block;
use crate::block_metadata::bound::BlockType;
use crate::block_metadata::parent::BlockParent;
use crate::config::Config;
use crate::span::Span;
use nom::Parser;
use nom::{IResult, branch::alt};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(tag = "category", rename_all = "lowercase")]
pub enum Block {
    Basic {
        attrs: BTreeMap<String, Vec<Span>>,
        r#type: BlockType,
        children: Vec<Block>,
        end_block: Option<Box<Block>>,
        flags: Vec<String>,
        kind: String,
    },
    CheckListItem,
    CheckList,
    CSS,
    CSV,
    End {
        attrs: BTreeMap<String, Vec<Span>>,
        bound: BlockType,
        children: Vec<Block>,
        flags: Vec<String>,
        kind: String,
    },
    Html,
    JavaScript,
    // TODO: Set up Json5 so that it's
    // top level is an `ok` or `error`
    // based of it it was able to be parsed
    // or not. Though about making it json5
    // but that might be a bridge to far?
    // need to think about that a bit.
    // If you do it, you'd have to figure out
    // how to deal with comments and such
    // in the AST which feels fraught.
    Json,
    List,
    ListItem,
    Olist,
    OlistItem,
    Raw {
        attrs: BTreeMap<String, Vec<Span>>,
        body: Option<String>,
        bound: BlockType,
        end_block: Option<Box<Block>>,
        flags: Vec<String>,
        kind: String,
    },
    Text {
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
        // Make sure to keep basic in the last slot
        |src| basic_block(src, config, parent),
    ))
    .parse(source)?;
    Ok((source, section))
}
