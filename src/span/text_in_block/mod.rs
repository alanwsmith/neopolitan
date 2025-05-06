#![allow(unused)]
use crate::block::Block;
use crate::block_metadata::parent::BlockParent;
use crate::config::Config;
use crate::span::Span;
use crate::span::shorthand::shorthand_span;
use crate::span::text::in_block::text_span_in_block;
use nom::Parser;
use nom::bytes::complete::is_not;
use nom::character::complete::line_ending;
use nom::character::complete::multispace0;
use nom::character::complete::not_line_ending;
use nom::character::complete::space0;
use nom::character::complete::space1;
use nom::combinator::not;
use nom::combinator::peek;
use nom::multi::many1;
use nom::sequence::terminated;
use nom::{IResult, branch::alt, bytes::complete::tag};

pub fn text_in_block<'a>(source: &'a str) -> IResult<&'a str, Span> {
    let (source, content) = many1(alt((
        is_not(" \r\n\t~`@^*_()[]{}<>"),
        space1.map(|_| " "),
        (tag("`"), not(tag("`"))).map(|_| "`"),
    )))
    .parse(source)?;
    Ok((
        source,
        Span::Text {
            content: content.join(""),
        },
    ))
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::block::Block;
    use crate::block_metadata::parent::BlockParent;
    use crate::helpers::*;
    use pretty_assertions::assert_eq;
    use serde_json::Value;
    use std::path::PathBuf;

    #[test]
    fn solo_list_item_text_span_tests() {
        let config = Config::default();
        let file_list = get_file_list(
            &PathBuf::from("src/span/text_in_block/tests"),
            &vec!["txt".to_string()],
        )
        .unwrap();
        for source_path in file_list {
            if let Ok(case) = get_test_data(&source_path) {
                let result = text_in_block(&case.source).unwrap();
                let left_content = (
                    &case.path,
                    serde_json::from_str::<Span>(&case.json).unwrap(),
                );
                let right_content = (&case.path, result.1);
                assert_eq!(left_content, right_content);
                dbg!(&case.remainder);
                let left_remainder = (&case.path, case.remainder);
                let right_remainder = (&case.path, result.0.to_string());
                assert_eq!(left_remainder, right_remainder);
            } else {
                assert!(false);
            }
        }
    }
}
