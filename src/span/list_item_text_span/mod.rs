#![allow(unused)]
use crate::block::Block;
use crate::block_metadata::parent::BlockParent;
use crate::config::Config;
use crate::span::Span;
use crate::span::shorthand::shorthand_span;
use crate::span::text::in_block::text_span_in_block;
use nom::Parser;
use nom::bytes::complete::is_not;
use nom::character::complete::multispace0;
use nom::character::complete::space1;
use nom::combinator::not;
use nom::multi::many1;
use nom::{IResult, branch::alt, bytes::complete::tag};

pub fn list_item_text_span<'a>(source: &'a str) -> IResult<&'a str, Span> {
    let (source, content) = many1(alt((is_not("\n"),))).parse(source)?;
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
            &PathBuf::from("src/span/list_item_text_span/tests"),
            &vec!["txt".to_string()],
        )
        .unwrap();
        for source_path in file_list {
            if let Ok(data) = get_test_data(&source_path) {
                let result = list_item_text_span(&data.0).unwrap();
                let left_content = (
                    data.3.clone(),
                    serde_json::from_str::<Span>(&data.1).unwrap(),
                );
                let right_content = (data.3.clone(), result.1);
                assert_eq!(left_content, right_content);
                let left_content = (data.3.clone(), data.2.trim_end());
                let right_content = (data.3.clone(), result.0);
                assert_eq!(left_content, right_content);
            } else {
                assert!(false);
            }
        }
    }
}
