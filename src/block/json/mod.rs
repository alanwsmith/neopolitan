pub mod open;
pub mod full;

use crate::block::Block;
use crate::block::json::full::json_block_full;
use crate::block::json::open::json_block_open;
use crate::block_metadata::parent::BlockParent;
use crate::config::Config;
use nom::Parser;
use nom::{IResult, branch::alt};

pub fn basic_block<'a>(
    source: &'a str,
    config: &'a Config,
    parent: &'a BlockParent,
) -> IResult<&'a str, Block> {
    let (source, section) = alt((
        |src| json_block_full(src, config, parent),
        |src| json_block_open(src, config, parent),
    ))
    .parse(source)?;
    Ok((source, section))
}
