use crate::config::Config;
use crate::section::Section;
use crate::section_bound::SectionBound;
use crate::section_category::SectionCategory;
use crate::section_parent::SectionParent;
use crate::span::code_span::code_span;
use crate::span::escaped_span::escaped_span;
use crate::span::span;
use crate::span::text_span::text_span;
use crate::span_strings::space0_line_ending_or_eof::space0_line_ending_or_eof;
use nom::Parser;
use nom::bytes::complete::is_not;
use nom::character::complete::multispace0;
use nom::character::complete::space1;
use nom::combinator::not;
use nom::multi::many1;
use nom::sequence::pair;
use nom::sequence::terminated;
use nom::{IResult, branch::alt, bytes::complete::tag, combinator::rest};
use serde::{Deserialize, Serialize};

pub fn text_block<'a>(
    source: &'a str,
    config: &'a Config,
    parent: &'a SectionParent,
    debug: bool,
) -> IResult<&'a str, Section> {
    let (source, _) = not(tag("--")).parse(source)?;

    let (source, spans) = many1(
        span,
        // alt((
        // text_span,
        // code_span,
        // escaped_span,
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
        // ))
    )
    .parse(source)?;
    let (source, _) = multispace0(source)?;

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

    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn basic_test() {
        let source = include_str!("test-data/text-block/basic-test.neo");
        let config = Config::default();
        let parent = SectionParent::Basic;
        let debug = false;
        let left = Section {
            category: SectionCategory::Block {
                spans: vec![Span::Text {
                    content: "this is some text with some lines".to_string(),
                }],
            },
            kind: "text-block".to_string(),
        };
        let right = text_block(source, &config, &parent, debug).unwrap().1;
        assert_eq!(left, right);
    }
}
