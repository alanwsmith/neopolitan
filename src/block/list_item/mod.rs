pub mod full;
pub mod open;

use crate::block::Block;
use crate::block::list_item::full::list_item_block_full;
use crate::block::list_item::open::list_item_block_open;
use crate::block_metadata::parent::BlockParent;
use crate::config::Config;
use nom::Parser;
use nom::{IResult, branch::alt};

// TODO: Make sure to test escaped ``-`` at the start
// of blocks keep the content in the existing item
// instead of making a new one (i.e. being able to
// start a paragraph with a ``-``)
//

pub fn list_item_block<'a>(
    source: &'a str,
    config: &'a Config,
    parent: &'a BlockParent,
    parent_kind: &'a str,
) -> IResult<&'a str, Block> {
    let (source, section) = alt((
        |src| list_item_block_full(src, config, parent, parent_kind),
        |src| list_item_block_open(src, config, parent, parent_kind),
    ))
    .parse(source)?;
    Ok((source, section))
}
