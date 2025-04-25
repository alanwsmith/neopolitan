#![allow(unused)]
use crate::neo_config::NeoConfig;
use crate::section::Section;
use crate::section_bound::SectionBound;
use crate::section_category::SectionCategory;
use crate::section_parent::SectionParent;
use crate::span_parsers::span_of_plain_text_for_block_paragraph::span_of_plain_text_for_block_paragraph;
use crate::span_strings::space0_line_ending_or_eof::space0_line_ending_or_eof;
use nom::Parser;
use nom::bytes::complete::is_not;
use nom::character::complete::space1;
use nom::combinator::not;
use nom::multi::many1;
use nom::sequence::pair;
use nom::sequence::terminated;
use nom::{IResult, branch::alt, bytes::complete::tag, combinator::rest};
use serde::{Deserialize, Serialize};

pub fn text_block<'a>(
    source: &'a str,
    config: &'a NeoConfig,
    parent: &'a SectionParent,
    debug: bool,
) -> IResult<&'a str, Section> {
    let (source, _) = not(tag("--")).parse(source)?;

    let (source, spans) = many1(alt((
        span_of_plain_text_for_block_paragraph,
        //     span_of_escaped_character,
        //     named_span,
        //     code_shorthand_span,
        //     emphasis_shorthand_span,
        //     footnote_shorthand_span,
        //     html_shorthand_span,
        //     image_shorthand_span,
        //     link_shorthand_span,
        //     mark_shorthand_span,
        //     strikethrough_shorthand_span,
        //     strong_shorthand_span,
        //     underline_shorthand_span,
    )))
    .parse(source)?;

    Ok((
        source,
        Section {
            category: SectionCategory::Block { spans },
            kind: "text-block".to_string(),
        },
    ))

    // dump_parser_if_debug(debug, dbg_header, source, 2);
    // let (source, _) = multispace0.context("").parse(source)?;
    // Ok((
    //     source,
    //     SectionV42::Block(BlockSectionV42 {
    //         children: vec![],
    //         kind: "paragraph".to_string(),
    //         spans,
    //         template_list: make_template_list(
    //             "paragraph",
    //             "paragraph",
    //             "default",
    //         ),
    //     }),
    // ))
}

#[cfg(test)]
mod test {
    use crate::span::Span;
    use crate::span::text::TextSpan;

    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn basic_test() {
        let source = include_str!("test-data/text-block/basic-test.neo");
        let config = NeoConfig::default();
        let parent = SectionParent::Basic;
        let debug = false;
        let left = Section {
            category: SectionCategory::Block {
                spans: vec![Span::Text(TextSpan {
                    kind: "text".to_string(),
                    text: "this is some text with some lines".to_string(),
                })],
            },
            kind: "text-block".to_string(),
        };
        let right = text_block(source, &config, &parent, debug).unwrap().1;
        assert_eq!(left, right);
    }
}
