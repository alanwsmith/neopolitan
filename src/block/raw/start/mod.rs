use crate::block::Block;
use crate::block::block;
use crate::block::end::end_block;
use crate::block::paragraph::paragraph_block;
use crate::block_metadata::block_metadata;
use crate::block_metadata::bound::BlockBound;
use crate::block_metadata::parent::BlockParent;
use crate::config::Config;
use crate::span_metadata::strings::space0_line_ending_or_eof::space0_line_ending_or_eof;
use nom::Parser;
use nom::branch::alt;
use nom::bytes::complete::is_not;
use nom::bytes::complete::take_until;
use nom::character::complete::multispace0;
use nom::character::complete::space1;
use nom::combinator::not;
use nom::combinator::rest;
use nom::multi::many0;
use nom::multi::many1;
use nom::sequence::pair;
use nom::sequence::terminated;
use nom::{IResult, bytes::complete::tag};

pub fn raw_block_start<'a>(
    source: &'a str,
    config: &'a Config,
    parent: &'a BlockParent,
) -> IResult<&'a str, Block> {
    let (source, _) = pair(tag("--"), space1).parse(source)?;
    let (source, kind) =
        terminated(is_not("/ \t\r\n"), (tag("/"), space0_line_ending_or_eof))
            .parse(source)?;
    if config.block_category_kinds.raw.contains(&kind.to_string()) {
        let (source, (attrs, flags)) = block_metadata(source, config, parent)?;
        let (source, _) = multispace0.parse(source)?;
        let (source, body_parts) = many1(alt((is_not("-"),))).parse(source)?;
        let joined_parts = body_parts.join("");

        let body = if joined_parts.is_empty() {
            None
        } else {
            Some(joined_parts.trim_end().to_string())
        };

        // let body = match trimmed_body {
        //     "".to_string() => None,
        //     _ => Some(trimmed_body.to_string()),
        // };

        // let (source, body_base) =
        //     alt((take_until(format!("-- /{}", kind).as_str()), rest))
        //         .parse(source)?;
        // let trimmed_body = body_base.trim_end();
        // let body = match trimmed_body {
        //     "" => None,
        //     _ => Some(trimmed_body.to_string()),
        // };

        let (source, end_block) =
            (|src| end_block(src, config, parent, kind)).parse(source)?;
        Ok((
            source,
            Block::Raw {
                attrs,
                body,
                bound: BlockBound::Start,
                end_block: Some(Box::new(end_block)),
                flags,
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
    use pretty_assertions::assert_eq;
    use std::collections::BTreeMap;

    #[test]
    fn solo_raw_block_start_test() {
        let source = "-- pre/\n\ndelta zulu alfa\n\n-- /pre";
        let config = Config::default();
        let parent = BlockParent::Page;
        let left = Block::Raw {
            attrs: BTreeMap::new(),
            bound: BlockBound::Start,
            body: Some("delta zulu alfa".to_string()),
            end_block: Some(Box::new(Block::End {
                attrs: BTreeMap::new(),
                bound: BlockBound::Full,
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

    // #[test]
    // fn solo_nested_block_start_test() {
    //     let source = "-- div/\n\n-- title\n\nwhiskey tango bravo\n\n-- /div";
    //     let config = Config::default();
    //     let parent = BlockParent::Page;
    //     let left = Block::Basic {
    //         attrs: BTreeMap::new(),
    //         bound: BlockBound::Start,
    //         children: vec![Block::Basic {
    //             attrs: BTreeMap::new(),
    //             bound: BlockBound::Full,
    //             children: vec![Block::Paragraph {
    //                 spans: vec![Span::Text {
    //                     content: "whiskey tango bravo".to_string(),
    //                 }],
    //             }],
    //             end_block: None,
    //             flags: vec![],
    //             kind: "title".to_string(),
    //         }],
    //         end_block: Some(Box::new(Block::End {
    //             attrs: BTreeMap::new(),
    //             bound: BlockBound::Full,
    //             children: vec![],
    //             flags: vec![],
    //             kind: "div-end".to_string(),
    //         })),
    //         flags: vec![],
    //         kind: "div".to_string(),
    //     };
    //     let right = raw_block_start(source, &config, &parent).unwrap().1;
    //     assert_eq!(left, right);
    // }

    //
}
