use super::text_block::text_block;
use crate::config::Config;
use crate::section_bound::SectionBound;
// use crate::section_category::SectionCategory;
use crate::section::Section;
use crate::section_metadata::section_metadata;
use crate::section_parent::SectionParent;
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
use std::collections::BTreeMap;

pub fn basic_section_full<'a>(
    source: &'a str,
    config: &'a Config,
    parent: &'a SectionParent,
    debug: bool,
) -> IResult<&'a str, Section> {
    let (source, _) = pair(tag("--"), space1).parse(source)?;
    let (source, kind) =
        terminated(is_not("/ \t\r\n"), space0_line_ending_or_eof)
            .parse(source)?;
    let (source, (attrs, flags)) =
        section_metadata(source, config, parent, debug)?;
    let (source, _) = multispace0.parse(source)?;
    let (source, children) =
        many0(|src| text_block(src, config, &SectionParent::Basic, debug))
            .parse(source)?;
    Ok((
        source,
        Section::Basic {
            attrs,
            bound: SectionBound::Full,
            children,
            end_section: None,
            flags,
            kind: kind.to_string(),
        },
    ))
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::span::Span;
    use pretty_assertions::assert_eq;

    #[test]
    fn basic_test() {
        let source =
            include_str!("test-data/basic-section-full/basic-test.neo");
        let config = Config::default();
        let parent = SectionParent::Page;
        let debug = false;
        let left = Section::Basic {
            attrs: BTreeMap::new(),
            bound: SectionBound::Full,
            children: vec![Section::Block {
                spans: vec![Span::Text {
                    content: "bravo foxtrot tango".to_string(),
                }],
            }],
            end_section: None,
            flags: vec![],
            kind: "title".to_string(),
        };
        let right = basic_section_full(source, &config, &parent, debug)
            .unwrap()
            .1;
        assert_eq!(left, right);
    }
}
