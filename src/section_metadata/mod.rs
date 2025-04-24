#![allow(unused)]
use crate::neo_config::NeoConfig;
use crate::section_attribute::SectionAttribute;
use crate::section_bound::SectionBound;
use crate::section_category::SectionCategory;
use crate::section_parent::SectionParent;
use crate::span::space0_line_ending_or_eof::space0_line_ending_or_eof;
use nom::Parser;
use nom::bytes::complete::is_not;
use nom::character::complete::space1;
use nom::sequence::pair;
use nom::sequence::terminated;
use nom::{IResult, branch::alt, bytes::complete::tag, combinator::rest};
use serde::{Deserialize, Serialize};

pub fn section_metadata<'a>(
    source: &'a str,
    config: &'a NeoConfig,
    parent: &'a SectionParent,
    debug: bool,
) -> IResult<&'a str, (Vec<SectionAttribute>, Vec<String>)> {
    Ok(("", (vec![], vec![])))
}
