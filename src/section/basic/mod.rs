pub mod full;
pub mod start;

use super::Section;
use crate::config::Config;
use crate::section::basic::full::basic_section_full;
use crate::section::parent::SectionParent;
use nom::Parser;
use nom::{IResult, branch::alt};

pub fn basic_section<'a>(
    source: &'a str,
    config: &'a Config,
    parent: &'a SectionParent,
    debug: bool,
) -> IResult<&'a str, Section> {
    let (source, section) =
        alt((|src| basic_section_full(src, config, parent, debug),))
            .parse(source)?;
    Ok((source, section))
}
