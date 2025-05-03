use crate::block::Block;
use crate::block::BlockParent;
use crate::block::text::text_block;
use crate::block_metadata::block_metadata;
use crate::config::Config;
use crate::span_metadata::strings::space0_line_ending_or_eof::space0_line_ending_or_eof;
use nom::IResult;
use nom::Parser;
use nom::bytes::complete::tag;
use nom::character::complete::multispace0;
use nom::character::complete::space1;
use nom::multi::many0;
use nom::sequence::terminated;

pub fn end_block<'a>(
    source: &'a str,
    config: &'a Config,
    parent: &'a BlockParent,
    kind: &str,
) -> IResult<&'a str, Block> {
    let (source, _) = (tag("--"), space1, tag("/")).parse(source)?;
    let (source, _) =
        terminated(tag(kind), space0_line_ending_or_eof).parse(source)?;
    let (source, metadata) = block_metadata(source, config, parent)?;
    let (source, _) = multispace0.parse(source)?;
    let (source, children) =
        many0(|src| text_block(src, config, &BlockParent::Basic))
            .parse(source)?;
    Ok((
        source,
        Block::End {
            attrs: metadata.attrs,
            children,
            flags: metadata.flags,
            kind: format!("{}-end", kind),
        },
    ))
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::span::Span;
    use pretty_assertions::assert_eq;
    use std::collections::BTreeMap;

    #[test]
    fn end_block_basic_test() {
        let source = r#"-- /some-end-block

bravo foxtrot tango"#;
        let config = Config::default();
        let parent = BlockParent::Page;
        let kind = "some-end-block";
        let left = Block::End {
            attrs: BTreeMap::new(),
            children: vec![Block::Text {
                spans: vec![Span::Text {
                    content: "bravo foxtrot tango".to_string(),
                }],
            }],
            flags: vec![],
            kind: "some-end-block-end".to_string(),
        };
        let right = end_block(source, &config, &parent, kind).unwrap().1;
        assert_eq!(left, right);
    }
}
