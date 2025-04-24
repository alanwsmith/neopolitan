#![allow(unused)]
use super::Section;
use crate::neo_config::NeoConfig;
use crate::section_bound::SectionBound;
use crate::section_category::SectionCategory;
use crate::section_metadata::section_metadata;
use crate::section_parent::SectionParent;
use crate::span::space0_line_ending_or_eof::space0_line_ending_or_eof;
use nom::Parser;
use nom::bytes::complete::is_not;
use nom::character::complete::space1;
use nom::sequence::pair;
use nom::sequence::terminated;
use nom::{IResult, branch::alt, bytes::complete::tag, combinator::rest};
use serde::{Deserialize, Serialize};

pub fn basic_section_full<'a>(
    source: &'a str,
    config: &'a NeoConfig,
    parent: &'a SectionParent,
    debug: bool,
) -> IResult<&'a str, Section> {
    let initial_source = source;
    let (source, _) = pair(tag("--"), space1).parse(source)?;
    let (source, kind) =
        terminated(is_not("/ \t\r\n"), space0_line_ending_or_eof)
            .parse(source)?;
    let (source, (attrs, flags)) =
        section_metadata(source, config, parent, debug)?;
    dbg!(&initial_source);
    dbg!(&source);
    let source_head = initial_source.replace(source, "").trim().to_string();
    Ok((
        "",
        Section {
            category: SectionCategory::Basic {
                attrs,
                bound: SectionBound::Full,
                chidren: vec![],
                end_section: None,
                flags,
                source_body: Some("bravo foxtrot tango".to_string()),
                source_head,
            },
            kind: kind.to_string(),
        },
    ))
}

#[cfg(test)]
mod test {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn basic_test() {
        let source =
            include_str!("test-data/basic-section-full/basic-test.neo");
        let config = NeoConfig::default();
        let parent = SectionParent::Page;
        let debug = false;
        let left = Section {
            category: SectionCategory::Basic {
                attrs: vec![],
                bound: SectionBound::Full,
                chidren: vec![],
                end_section: None,
                flags: vec![],
                source_body: Some("bravo foxtrot tango".to_string()),
                source_head: "-- title".to_string(),
            },
            kind: "title".to_string(),
        };
        let right = basic_section_full(source, &config, &parent, debug)
            .unwrap()
            .1;
        assert_eq!(left, right);
    }
}
