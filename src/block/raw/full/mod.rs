use crate::block::Block;
use crate::block_metadata::block_metadata;
use crate::block_metadata::parent::BlockParent;
use crate::config::Config;
use crate::span_metadata::strings::space0_line_ending_or_eof::space0_line_ending_or_eof;
use nom::Parser;
use nom::branch::alt;
use nom::bytes::complete::is_not;
use nom::bytes::complete::take_until;
use nom::character::complete::multispace0;
use nom::character::complete::space0;
use nom::character::complete::space1;
use nom::combinator::rest;
use nom::sequence::terminated;
use nom::{IResult, bytes::complete::tag};

pub fn raw_block_full<'a>(
    source: &'a str,
    config: &'a Config,
    parent: &'a BlockParent,
) -> IResult<&'a str, Block> {
    let (source, _) = (space0, tag("--"), space1).parse(source)?;
    let (source, kind) =
        terminated(is_not("/ \t\r\n"), space0_line_ending_or_eof)
            .parse(source)?;
    if config.block_category_kinds.raw.contains(&kind.to_string()) {
        let (source, metadata) = block_metadata(source, config, parent)?;
        // TODO: Update this so it doesn't slup initil
        // empty spaces up on lines that don't start
        // at the first character
        let (source, _) = multispace0.parse(source)?;
        let (source, body_base) =
            alt((take_until("-- "), rest)).parse(source)?;
        let trimmed_body = body_base.trim_end();
        let body = match trimmed_body {
            "" => None,
            _ => Some(trimmed_body.to_string()),
        };
        Ok((
            source,
            Block::Raw {
                attrs: metadata.attrs,
                body,
                end_block: None,
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
    use pretty_assertions::assert_eq;
    use std::collections::BTreeMap;

    #[test]
    fn raw_basic_test() {
        let source = r#"-- pre 
alfa delta whiskey"#;
        let config = Config::default();
        let parent = BlockParent::Page;
        let left = Block::Raw {
            attrs: BTreeMap::new(),
            body: Some("alfa delta whiskey".to_string()),
            end_block: None,
            flags: vec![],
            kind: "pre".to_string(),
        };
        let right = raw_block_full(source, &config, &parent).unwrap().1;
        assert_eq!(left, right);
    }

    #[test]
    fn raw_basic_test_chomp_leading_line_space() {
        let source = r#"  -- pre 
alfa delta whiskey"#;
        let config = Config::default();
        let parent = BlockParent::Page;
        let left = Block::Raw {
            attrs: BTreeMap::new(),
            body: Some("alfa delta whiskey".to_string()),
            end_block: None,
            flags: vec![],
            kind: "pre".to_string(),
        };
        let right = raw_block_full(source, &config, &parent).unwrap().1;
        assert_eq!(left, right);
    }

//
}
