use crate::block::Block;
use crate::block_metadata::parent::BlockParent;
use crate::config::Config;
use crate::span::shorthand::shorthand_span;
use crate::span::text::in_block::text_span_in_block;
use nom::Parser;
use nom::character::complete::multispace0;
use nom::combinator::not;
use nom::multi::many1;
use nom::{IResult, branch::alt, bytes::complete::tag};

pub fn paragraph_block<'a>(
    source: &'a str,
    _config: &'a Config,
    _parent: &'a BlockParent,
) -> IResult<&'a str, Block> {
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

    Ok((source, Block::Paragraph { spans }))

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
    use super::*;
    use crate::span::Span;
    use pretty_assertions::assert_eq;

    #[test]
    fn basic_test() {
        let source = r#"this is some 
text with some lines"#;
        let config = Config::default();
        let parent = BlockParent::Basic;
        let left = Block::Paragraph {
            spans: vec![Span::Text {
                content: "this is some text with some lines".to_string(),
            }],
        };
        let right = paragraph_block(source, &config, &parent).unwrap().1;
        assert_eq!(left, right);
    }
}
