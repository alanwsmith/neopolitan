use crate::block::Block;
use crate::block::end::end_section;
use crate::block::paragraph::paragraph_block;
use crate::block_metadata::bound::BlockBound;
use crate::block_metadata::parent::BlockParent;
use crate::block_metadata::block_metadata;
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

pub fn basic_section_start<'a>(
    source: &'a str,
    config: &'a Config,
    parent: &'a BlockParent,
    debug: bool,
) -> IResult<&'a str, Block> {
    let (source, _) = pair(tag("--"), space1).parse(source)?;
    let (source, kind) =
        terminated(is_not("/ \t\r\n"), (tag("/"), space0_line_ending_or_eof))
            .parse(source)?;
    let (source, (attrs, flags)) =
        block_metadata(source, config, parent, debug)?;
    let (source, _) = multispace0.parse(source)?;
    let (source, children) =
        many0(|src| paragraph_block(src, config, &BlockParent::Basic, debug))
            .parse(source)?;
    let (source, end_section) =
        (|src| end_section(src, config, parent, kind)).parse(source)?;
    Ok((
        source,
        Block::Basic {
            attrs,
            bound: BlockBound::Start,
            children,
            end_block: Some(Box::new(end_section)),
            flags,
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

    #[test]
    fn solo_basic_section_start_test() {
        let source = r#"-- aside/

delta zulu alfa

-- /aside"#;
        let config = Config::default();
        let parent = BlockParent::Page;
        let debug = false;
        let left = Block::Basic {
            attrs: BTreeMap::new(),
            bound: BlockBound::Start,
            children: vec![Block::Paragraph {
                spans: vec![Span::Text {
                    content: "delta zulu alfa".to_string(),
                }],
            }],
            end_block: Some(Box::new(Block::End {
                attrs: BTreeMap::new(),
                bound: BlockBound::Full,
                children: vec![],
                flags: vec![],
                kind: "aside-end".to_string(),
            })),
            flags: vec![],
            kind: "aside".to_string(),
        };
        let right = basic_section_start(source, &config, &parent, debug)
            .unwrap()
            .1;
        assert_eq!(left, right);
    }
}
