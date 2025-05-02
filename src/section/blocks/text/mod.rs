use crate::config::Config;
use crate::section::Section;
use crate::section::bound::SectionBound;
use crate::section::parent::SectionParent;
use crate::span::shorthand::shorthand_span;
use crate::span::strings::space0_line_ending_or_eof::space0_line_ending_or_eof;
use crate::span::text::in_block::text_span_in_block;
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

    let (source, spans) = many1(alt((
        text_span_in_block,
        shorthand_span,
        // alt((
        // text_span_in_block,
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
    )))
    .parse(source)?;
    let (source, _) = multispace0(source)?;

    Ok((source, Section::Block { spans }))

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
        let source = r#"this is some 
text with some lines"#;
        let config = Config::default();
        let parent = SectionParent::Basic;
        let debug = false;
        let left = Section::Block {
            spans: vec![Span::Text {
                content: "this is some text with some lines".to_string(),
            }],
        };
        let right = text_block(source, &config, &parent, debug).unwrap().1;
        assert_eq!(left, right);
    }
}
