pub mod full;
pub mod start;

use crate::block::Block;
use crate::block::raw::full::raw_block_full;
use crate::block::raw::start::raw_block_start;
use crate::block_metadata::parent::BlockParent;
use crate::config::Config;
use nom::Parser;
use nom::{IResult, branch::alt};

pub fn raw_block<'a>(
    source: &'a str,
    config: &'a Config,
    parent: &'a BlockParent,
) -> IResult<&'a str, Block> {
    let (source, section) = alt((
        |src| raw_block_full(src, config, parent),
        |src| raw_block_start(src, config, parent),
    ))
    .parse(source)?;
    Ok((source, section))
}
