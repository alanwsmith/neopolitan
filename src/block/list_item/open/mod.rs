#![allow(unused)]
use crate::block::Block;
use crate::block::block;
use crate::block::end::end_block;
use crate::block::text_block::text_block;
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
use nom::multi::many0;
use nom::sequence::terminated;
use nom::{IResult, bytes::complete::tag};

pub fn list_item_block_open<'a>(
    source: &'a str,
    config: &'a Config,
    parent: &'a BlockParent,
    parent_kind: &'a str,
) -> IResult<&'a str, Block> {
    let (source, _) = (space0, tag("--"), space1).parse(source)?;
    let (source, kind) =
        terminated(is_not("/ \t\r\n"), (tag("/"), space0_line_ending_or_eof))
            .parse(source)?;
    // let (source, (attrs, flags)) = block_metadata(source, config, parent)?;
    let (source, metadata) = block_metadata(source, config, parent)?;
    let (source, _) = multispace0.parse(source)?;
    let (source, children) = many0(alt((
        |src| text_block(src, config, &BlockParent::Basic),
        |src| block(src, config, &BlockParent::Basic),
    )))
    .parse(source)?;
    let (source, end_block) =
        (|src| end_block(src, config, parent, kind)).parse(source)?;
    Ok((
        source,
        Block::ListItem {
            children,
            kind: kind.to_string(),
        },
    ))
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::span::Span;
    use pretty_assertions::assert_eq;
    use std::collections::BTreeMap;

    // #[test]
    // fn basic_block_start_test() {
    //     let source = r#"-- aside/
    // delta zulu alfa
    // -- /aside"#;
    //     let config = Config::default();
    //     let parent = BlockParent::Page;
    //     let left = Block::Basic {
    //         attrs: BTreeMap::new(),
    //         children: vec![Block::TextBlock {
    //             kind: "text-block".to_string(),
    //             spans: vec![Span::Text {
    //                 content: "delta zulu alfa".to_string(),
    //             }],
    //         }],
    //         end_block: Some(Box::new(Block::End {
    //             attrs: BTreeMap::new(),
    //             children: vec![],
    //             flags: vec![],
    //             kind: "aside-end".to_string(),
    //         })),
    //         flags: vec![],
    //         kind: "aside".to_string(),
    //     };
    //     let right = basic_block_start(source, &config, &parent).unwrap().1;
    //     assert_eq!(left, right);
    // }

    // #[test]
    // fn nested_block_start_test() {
    //     let source = "-- div/\n\n-- title\n\nwhiskey tango bravo\n\n-- /div";
    //     let config = Config::default();
    //     let parent = BlockParent::Page;
    //     let left = Block::Basic {
    //         attrs: BTreeMap::new(),
    //         children: vec![Block::Basic {
    //             attrs: BTreeMap::new(),
    //             children: vec![Block::TextBlock {
    //                 kind: "text-block".to_string(),
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
    //             children: vec![],
    //             flags: vec![],
    //             kind: "div-end".to_string(),
    //         })),
    //         flags: vec![],
    //         kind: "div".to_string(),
    //     };
    //     let right = basic_block_start(source, &config, &parent).unwrap().1;
    //     assert_eq!(left, right);
    // }

    // #[test]
    // fn nested_block_start_test_chomping_leading_line_spaces() {
    //     let source =
    //         "  -- div/\n\n  -- title\n\nwhiskey tango bravo\n\n  -- /div";
    //     let config = Config::default();
    //     let parent = BlockParent::Page;
    //     let left = Block::Basic {
    //         attrs: BTreeMap::new(),
    //         children: vec![Block::Basic {
    //             attrs: BTreeMap::new(),
    //             children: vec![Block::TextBlock {
    //                 kind: "text-block".to_string(),
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
    //             children: vec![],
    //             flags: vec![],
    //             kind: "div-end".to_string(),
    //         })),
    //         flags: vec![],
    //         kind: "div".to_string(),
    //     };
    //     let right = basic_block_start(source, &config, &parent).unwrap().1;
    //     assert_eq!(left, right);
    // }

    //
}
