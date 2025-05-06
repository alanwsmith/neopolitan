pub mod full;
pub mod open;

use crate::block::Block;
use crate::block::list::full::list_block_full;
use crate::block::list::open::list_block_open;
use crate::block_metadata::parent::BlockParent;
use crate::config::Config;
use nom::Parser;
use nom::{IResult, branch::alt};

pub fn list_block<'a>(
    source: &'a str,
    config: &'a Config,
    parent: &'a BlockParent,
) -> IResult<&'a str, Block> {
    let (source, section) = alt((
        |src| list_block_full(src, config, parent),
        |src| list_block_open(src, config, parent),
    ))
    .parse(source)?;
    Ok((source, section))
}
