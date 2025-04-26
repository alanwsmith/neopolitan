#![allow(unused)]
pub mod basic_section;
pub mod basic_section_full;
pub mod text_block;

use crate::config::Config;
use crate::section::basic_section::basic_section;
use crate::section_category::SectionCategory;
use crate::section_parent::SectionParent;
use nom::Parser;
use nom::{IResult, branch::alt, bytes::complete::tag};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub struct Section {
    pub category: SectionCategory,
    pub kind: String,
}

pub fn parse_section<'a>(
    source: &'a str,
    config: &'a Config,
    parent: &'a SectionParent,
    debug: bool,
) -> IResult<&'a str, Section> {
    let (source, section) = alt((
        |src| basic_section(src, config, parent, debug),
        |src| basic_section(src, config, parent, debug),
    ))
    .parse(source)?;
    Ok((source, section))
}
