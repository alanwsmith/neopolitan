use crate::block::Block;
use crate::block::BlockParent;
use crate::block::paragraph::paragraph_block;
use crate::block_metadata::bound::BlockBound;
use crate::block_metadata::section_metadata;
use crate::config::Config;
use crate::span_metadata::strings::space0_line_ending_or_eof::space0_line_ending_or_eof;
use nom::IResult;
use nom::Parser;
use nom::bytes::complete::tag;
use nom::character::complete::multispace0;
use nom::character::complete::space1;
use nom::multi::many0;
use nom::sequence::terminated;

pub fn end_section<'a>(
    source: &'a str,
    config: &'a Config,
    parent: &'a BlockParent,
    kind: &str,
) -> IResult<&'a str, Block> {
    dbg!(&source);
    let (source, _) = (tag("--"), space1, tag("/")).parse(source)?;
    let (source, _) =
        terminated(tag(kind), space0_line_ending_or_eof).parse(source)?;
    let (source, (attrs, flags)) =
        section_metadata(source, config, parent, false)?;
    let (source, _) = multispace0.parse(source)?;
    let (source, children) =
        many0(|src| paragraph_block(src, config, &BlockParent::Basic, false))
            .parse(source)?;
    Ok((
        source,
        Block::End {
            attrs,
            bound: BlockBound::Full,
            children,
            flags,
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
    fn solo_end_section_basic_test() {
        let source = r#"-- /some-end-section

bravo foxtrot tango"#;
        let config = Config::default();
        let parent = BlockParent::Page;
        let kind = "some-end-section";
        let left = Block::End {
            attrs: BTreeMap::new(),
            bound: BlockBound::Full,
            children: vec![Block::Paragraph {
                spans: vec![Span::Text {
                    content: "bravo foxtrot tango".to_string(),
                }],
            }],
            flags: vec![],
            kind: "some-end-section-end".to_string(),
        };
        let right = end_section(source, &config, &parent, kind).unwrap().1;
        assert_eq!(left, right);
    }
}
