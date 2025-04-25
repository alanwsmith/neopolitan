#![allow(unused)]
use super::Section;
use super::text_block::text_block;
use crate::neo_config::NeoConfig;
use crate::section_bound::SectionBound;
use crate::section_category::SectionCategory;
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

pub fn basic_section_full<'a>(
    source: &'a str,
    config: &'a NeoConfig,
    parent: &'a SectionParent,
    debug: bool,
) -> IResult<&'a str, Section> {
    let initial_head = source;
    let (source, _) = pair(tag("--"), space1).parse(source)?;
    let (source, kind) =
        terminated(is_not("/ \t\r\n"), space0_line_ending_or_eof)
            .parse(source)?;
    let (source, (attrs, flags)) =
        section_metadata(source, config, parent, debug)?;
    let source_head = initial_head.replace(source, "").trim().to_string();
    let (source, _) = multispace0.parse(source)?;
    let initial_body = source;
    let (source, children) =
        many0(|src| text_block(src, config, &SectionParent::Basic, debug))
            .parse(source)?;
    let tmp_source_body = initial_body.replace(source, "").trim().to_string();
    let source_body = if tmp_source_body != "" {
        Some(tmp_source_body)
    } else {
        None
    };
    Ok((
        "",
        Section {
            category: SectionCategory::Basic {
                attrs,
                bound: SectionBound::Full,
                children,
                end_section: None,
                flags,
                source_body,
                source_head,
            },
            kind: kind.to_string(),
        },
    ))
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::span::{Span, text::TextSpan};
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
                children: vec![Section {
                    category: SectionCategory::Block {
                        spans: vec![Span::Text(TextSpan {
                            kind: "text".to_string(),
                            text: "bravo foxtrot tango".to_string(),
                        })],
                    },
                    kind: "text-block".to_string(),
                }],
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
