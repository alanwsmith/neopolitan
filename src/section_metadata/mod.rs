#![allow(unused)]
use crate::neo_config::NeoConfig;
use crate::section_attribute::SectionAttribute;
use crate::section_attribute::raw_section_attribute;
use crate::section_bound::SectionBound;
use crate::section_category::SectionCategory;
use crate::section_flag::raw_section_flag;
use crate::section_parent::SectionParent;
use crate::span::Span;
use crate::span_strings::space0_line_ending_or_eof::space0_line_ending_or_eof;
use nom::Parser;
use nom::bytes::complete::is_not;
use nom::character::complete::multispace0;
use nom::character::complete::space1;
use nom::multi::many0;
use nom::sequence::pair;
use nom::sequence::terminated;
use nom::{IResult, branch::alt, bytes::complete::tag, combinator::rest};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub enum RawSectionMetaData {
    Attribtue { key: String, spans: Vec<Span> },
    Flag { string: String },
}

pub fn section_metadata<'a>(
    source: &'a str,
    config: &'a NeoConfig,
    parent: &'a SectionParent,
    debug: bool,
) -> IResult<&'a str, (Vec<SectionAttribute>, Vec<String>)> {
    let (source, raw_metadata) = many0(alt((
        |src| raw_section_flag(src, config, parent, debug),
        |src| raw_section_attribute(src, config, parent, debug),
    )))
    .parse(source)?;
    let flags = raw_metadata
        .iter()
        .filter_map(|metadata| match metadata {
            RawSectionMetaData::Flag { string } => Some(string.clone()),
            _ => None,
        })
        .collect::<Vec<String>>();
    Ok((source, (vec![], flags)))
}

#[cfg(test)]
mod test {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    pub fn section_metadata_basic_test() {
        let config = &NeoConfig::default();
        let source = "-- test-flag\n\n";
        let parent = &SectionParent::Basic;
        let debug = false;
        let left = (vec![], vec!["test-flag".to_string()]);
        let right = section_metadata(source, config, parent, debug).unwrap().1;
        assert_eq!(left, right);
    }
}
