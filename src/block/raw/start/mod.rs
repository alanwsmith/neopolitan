use crate::block::Block;
use crate::block::end::end_block;
use crate::block_metadata::block_metadata;
use crate::block_metadata::parent::BlockParent;
use crate::config::Config;
use crate::span_metadata::strings::space0_line_ending_or_eof::space0_line_ending_or_eof;
use nom::Parser;
use nom::branch::alt;
use nom::bytes::complete::is_not;
use nom::character::complete::multispace0;
use nom::character::complete::space0;
use nom::character::complete::space1;
use nom::combinator::not;
use nom::combinator::recognize;
use nom::multi::many1;
use nom::sequence::terminated;
use nom::{IResult, bytes::complete::tag};

pub fn raw_block_start<'a>(
    source: &'a str,
    config: &'a Config,
    parent: &'a BlockParent,
) -> IResult<&'a str, Block> {
    let (source, _) = (space0, tag("--"), space1).parse(source)?;
    let (source, kind) =
        terminated(is_not("/ \t\r\n"), (tag("/"), space0_line_ending_or_eof))
            .parse(source)?;
    if config.block_category_kinds.raw.contains(&kind.to_string()) {
        let (source, metadata) = block_metadata(source, config, parent)?;
        let (source, _) = multispace0.parse(source)?;
        let (source, body_parts) = many1(alt((
            is_not("-"),
            recognize((
                tag("-"),
                not((tag("-"), space1, tag(format!("/{}", kind).as_str()))),
            )),
        )))
        .parse(source)?;
        let joined_parts = body_parts.join("");
        let body = if joined_parts.is_empty() {
            None
        } else {
            Some(joined_parts.trim_end().to_string())
        };
        let (source, end_block) =
            (|src| end_block(src, config, parent, kind)).parse(source)?;
        Ok((
            source,
            Block::Raw {
                attrs: metadata.attrs,
                body,
                end_block: Some(Box::new(end_block)),
                flags: metadata.flags,
                kind: kind.to_string(),
            },
        ))
    } else {
        Err(nom::Err::Error(nom::error::Error {
            input: source,
            code: nom::error::ErrorKind::Tag,
        }))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::span::Span;
    use pretty_assertions::assert_eq;
    use std::collections::BTreeMap;

    #[test]
    fn raw_block_start_test() {
        let source = "-- pre/\n\ndelta zulu alfa\n\n-- /pre";
        let config = Config::default();
        let parent = BlockParent::Page;
        let left = Block::Raw {
            attrs: BTreeMap::new(),
            body: Some("delta zulu alfa".to_string()),
            end_block: Some(Box::new(Block::End {
                attrs: BTreeMap::new(),
                children: vec![],
                flags: vec![],
                kind: "pre-end".to_string(),
            })),
            flags: vec![],
            kind: "pre".to_string(),
        };
        let right = raw_block_start(source, &config, &parent).unwrap().1;
        assert_eq!(left, right);
    }


    #[test]
    fn raw_block_start_test_chomp_leading_line_spaces() {
        let source = "  -- pre/\n\ndelta zulu alfa\n\n-- /pre";
        let config = Config::default();
        let parent = BlockParent::Page;
        let left = Block::Raw {
            attrs: BTreeMap::new(),
            body: Some("delta zulu alfa".to_string()),
            end_block: Some(Box::new(Block::End {
                attrs: BTreeMap::new(),
                children: vec![],
                flags: vec![],
                kind: "pre-end".to_string(),
            })),
            flags: vec![],
            kind: "pre".to_string(),
        };
        let right = raw_block_start(source, &config, &parent).unwrap().1;
        assert_eq!(left, right);
    }

    #[test]
    fn raw_block_start_dash_in_content_test() {
        let source = "-- raw/\n\ndelta-zulu alfa\n\n-- /raw";
        let config = Config::default();
        let parent = BlockParent::Page;
        let left = Block::Raw {
            attrs: BTreeMap::new(),
            body: Some("delta-zulu alfa".to_string()),
            end_block: Some(Box::new(Block::End {
                attrs: BTreeMap::new(),
                children: vec![],
                flags: vec![],
                kind: "raw-end".to_string(),
            })),
            flags: vec![],
            kind: "raw".to_string(),
        };
        let right = raw_block_start(source, &config, &parent).unwrap().1;
        assert_eq!(left, right);
    }

    #[test]
    fn raw_block_start_nested_block_start_test() {
        let source = "-- code/\n\n-- title\n\nwhiskey tango bravo\n\n-- /code";
        let config = Config::default();
        let parent = BlockParent::Page;
        let left = Block::Raw {
            attrs: BTreeMap::new(),
            body: Some("-- title\n\nwhiskey tango bravo".to_string()),
            end_block: Some(Box::new(Block::End {
                attrs: BTreeMap::new(),
                children: vec![],
                flags: vec![],
                kind: "code-end".to_string(),
            })),
            flags: vec![],
            kind: "code".to_string(),
        };
        let right = raw_block_start(source, &config, &parent).unwrap().1;
        assert_eq!(left, right);
    }

    #[test]
    fn raw_block_start_nested_with_end_block_test() {
        let source = "-- code/\n\n-- title\n\nwhiskey tango bravo\n\n-- /code\n\nsierra kilo";
        let config = Config::default();
        let parent = BlockParent::Page;
        let left = Block::Raw {
            attrs: BTreeMap::new(),
            body: Some("-- title\n\nwhiskey tango bravo".to_string()),
            end_block: Some(Box::new(Block::End {
                attrs: BTreeMap::new(),
                children: vec![Block::TextBlock {
                    spans: vec![Span::Text {
                        content: "sierra kilo".to_string(),
                    }],
                }],
                flags: vec![],
                kind: "code-end".to_string(),
            })),
            flags: vec![],
            kind: "code".to_string(),
        };
        let right = raw_block_start(source, &config, &parent).unwrap().1;
        assert_eq!(left, right);
    }

    //
}
