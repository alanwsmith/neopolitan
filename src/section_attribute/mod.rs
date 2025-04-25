#![allow(unused)]
use crate::neo_config::NeoConfig;
use crate::section_bound::SectionBound;
use crate::section_category::SectionCategory;
use crate::section_parent::SectionParent;
use crate::span_strings::space0_line_ending_or_eof::space0_line_ending_or_eof;
use nom::Parser;
use nom::bytes::complete::is_not;
use nom::character::complete::space1;
use nom::sequence::pair;
use nom::sequence::terminated;
use nom::{IResult, branch::alt, bytes::complete::tag, combinator::rest};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub struct SectionAttribute {
    key: String,
}

pub fn section_attribute<'a>(
    source: &'a str,
    config: &'a NeoConfig,
    parent: &'a SectionParent,
    debug: bool,
) -> IResult<&'a str, SectionAttribute> {
    Ok((
        "",
        SectionAttribute {
            key: "TODO".to_string(),
        },
    ))
}
