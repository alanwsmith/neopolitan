use crate::block::Block;
use crate::block::text::text_block;
use crate::block_metadata::block_metadata;
use crate::block_metadata::bound::BlockType;
use crate::block_metadata::parent::BlockParent;
use crate::config::Config;
use crate::span_metadata::strings::space0_line_ending_or_eof::space0_line_ending_or_eof;
use nom::Parser;
use nom::bytes::complete::is_not;
use nom::character::complete::multispace0;
use nom::character::complete::space1;
use nom::multi::many0;
use nom::sequence::pair;
use nom::sequence::terminated;
use nom::{IResult, bytes::complete::tag};

pub fn basic_block_full<'a>(
    source: &'a str,
    config: &'a Config,
    parent: &'a BlockParent,
) -> IResult<&'a str, Block> {
    let (source, _) = pair(tag("--"), space1).parse(source)?;
    let (source, kind) =
        terminated(is_not("/ \t\r\n"), space0_line_ending_or_eof)
            .parse(source)?;
    let (source, metadata) = block_metadata(source, config, parent)?;
    let (source, _) = multispace0.parse(source)?;
    let (source, children) =
        many0(|src| text_block(src, config, &BlockParent::Basic))
            .parse(source)?;
    Ok((
        source,
        Block::Basic {
            attrs: metadata.attrs,
            children,
            end_block: None,
            flags: metadata.flags,
            kind: kind.to_string(),
        },
    ))
}

// pub fn basic_block_full<'a>(
//     source: &'a str,
//     config: &'a Config,
//     parent: &'a BlockParent,
// ) -> IResult<&'a str, Block> {
//     let (source, _) = pair(tag("--"), space1).parse(source)?;
//     let (source, kind) =
//         terminated(is_not("/ \t\r\n"), space0_line_ending_or_eof)
//             .parse(source)?;
//     let (source, (attrs, flags)) = block_metadata(source, config, parent)?;
//     let (source, _) = multispace0.parse(source)?;
//     let (source, children) =
//         many0(|src| paragraph_block(src, config, &BlockParent::Basic))
//             .parse(source)?;
//     Ok((
//         source,
//         Block::Basic {
//             attrs,
//             bound: BlockBound::Full,
//             children,
//             end_block: None,
//             flags,
//             kind: kind.to_string(),
//         },
//     ))
// }

#[cfg(test)]
mod test {
    use super::*;
    use crate::span::Span;
    use pretty_assertions::assert_eq;
    use std::collections::BTreeMap;

    #[test]
    fn basic_test() {
        let source = r#"-- title

bravo foxtrot tango"#;
        let config = Config::default();
        let parent = BlockParent::Page;
        let left = Block::Basic {
            attrs: BTreeMap::new(),
            children: vec![Block::Text {
                spans: vec![Span::Text {
                    content: "bravo foxtrot tango".to_string(),
                }],
            }],
            end_block: None,
            flags: vec![],
            kind: "title".to_string(),
        };
        let right = basic_block_full(source, &config, &parent).unwrap().1;
        assert_eq!(left, right);
    }
}
