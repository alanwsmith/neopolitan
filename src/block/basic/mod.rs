pub mod full;
pub mod start;

use crate::block::Block;
use crate::block::basic::full::basic_section_full;
use crate::block_metadata::parent::SectionParent;
use crate::config::Config;
use nom::Parser;
use nom::{IResult, branch::alt};

pub fn basic_section<'a>(
    source: &'a str,
    config: &'a Config,
    parent: &'a SectionParent,
    debug: bool,
) -> IResult<&'a str, Block> {
    let (source, section) =
        alt((|src| basic_section_full(src, config, parent, debug),))
            .parse(source)?;
    Ok((source, section))
}
