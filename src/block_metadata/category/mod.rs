// TODO: Figure out a better way to handle this
// since you aren't really pulling in the data
// and I think it's only used in the config
//
// So, gonna try to deprecate
//

use crate::block::Block;
use crate::{block_metadata::bound::SectionBound, span::Span};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
// #[serde(untagged)]
#[serde(tag = "type", content = "content", rename_all = "lowercase")]
pub enum SectionCategory {
    Basic {
        attrs: BTreeMap<String, Vec<Vec<Span>>>,
        bound: SectionBound,
        children: Vec<Block>,
        end_section: Option<Box<Block>>,
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
