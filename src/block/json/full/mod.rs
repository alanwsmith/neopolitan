use crate::block::Block;
use crate::block::JsonData;
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

pub fn json_block_full<'a>(
    source: &'a str,
    config: &'a Config,
    parent: &'a BlockParent,
) -> IResult<&'a str, Block> {
    let (source, _) = (space0, tag("--"), space1).parse(source)?;
    let (source, kind) =
        terminated(is_not("/ \t\r\n"), space0_line_ending_or_eof)
            .parse(source)?;
    if config.block_category_kinds.json.contains(&kind.to_string()) {
        let (source, metadata) = block_metadata(source, config, parent)?;
        let (source, _) = multispace0.parse(source)?;
        let (source, body_base) =
            alt((take_until("-- "), rest)).parse(source)?;
        match serde_json::from_str(body_base) {
            Ok(json) => Ok((
                source,
                Block::Json {
                    attrs: metadata.attrs,
                    data: JsonData::Ok(json),
                    end_block: None,
                    flags: metadata.flags,
                    kind: kind.to_string(),
                },
            )),
            Err(e) => Ok((
                source,
                Block::Json {
                    attrs: metadata.attrs,
                    data: JsonData::Error(e.to_string()),
                    end_block: None,
                    flags: metadata.flags,
                    kind: kind.to_string(),
                },
            )),
        }
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
    fn json_block_test() {
        let source = r#"-- json

{ "alfa": "bravo" }"#;
        let config = Config::default();
        let parent = BlockParent::Page;
        let left = Block::Json {
            attrs: BTreeMap::new(),
            data: JsonData::Ok(
                serde_json::from_str(r#"{"alfa": "bravo"}"#).unwrap(),
            ),
            end_block: None,
            flags: vec![],
            kind: "json".to_string(),
        };
        let right = json_block_full(source, &config, &parent).unwrap().1;
        assert_eq!(left, right);
    }

    #[test]
    fn json_block_test_chomp_leading_line_space() {
        let source = r#"    -- json

{ "tango": "foxtrot" }"#;
        let config = Config::default();
        let parent = BlockParent::Page;
        let left = Block::Json {
            attrs: BTreeMap::new(),
            data: JsonData::Ok(
                serde_json::from_str(r#"{"tango": "foxtrot"}"#).unwrap(),
            ),
            end_block: None,
            flags: vec![],
            kind: "json".to_string(),
        };
        let right = json_block_full(source, &config, &parent).unwrap().1;
        assert_eq!(left, right);
    }

    #[test]
    fn json_block_error_test() {
        let source = r#"-- json

xxx"#;
        let config = Config::default();
        let parent = BlockParent::Page;
        let left = Block::Json {
            attrs: BTreeMap::new(),
            data: JsonData::Error(
                "expected value at line 1 column 1".to_string(),
            ),
            end_block: None,
            flags: vec![],
            kind: "json".to_string(),
        };
        let right = json_block_full(source, &config, &parent).unwrap().1;
        assert_eq!(left, right);
    }

    //
}
