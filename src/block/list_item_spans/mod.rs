#![allow(unused)]
use crate::block::Block;
use crate::block_metadata::parent::BlockParent;
use crate::config::Config;
use crate::span::Span;
use crate::span::shorthand::shorthand_span;
use crate::span::single_line_ending::single_line_ending;
use crate::span::text::in_block::text_span_in_block;
use crate::span::text_in_block::text_in_block;
use nom::Parser;
use nom::character::complete::multispace0;
use nom::character::complete::space1;
use nom::combinator::not;
use nom::multi::many0;
use nom::multi::many1;
use nom::{IResult, branch::alt, bytes::complete::tag};

pub fn list_item_spans<'a>(
    source: &'a str,
    _config: &'a Config,
    _parent: &'a BlockParent,
    parent_kind: &'a str,
) -> IResult<&'a str, Block> {
    let (source, spans) =
        many0(alt((text_in_block, single_line_ending))).parse(source)?;
    Ok((
        source,
        Block::ListItemSpans {
            kind: format!("{}-spans", parent_kind),
            spans,
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
    #[ignore]
    fn solo_text_in_block_tests() {
        let config = Config::default();
        let file_list = get_file_list(
            &PathBuf::from("src/block/list_item_spans/tests"),
            &vec!["neotest".to_string()],
        )
        .unwrap();
        for source_path in file_list {
            match get_test_data(&source_path) {
                TestCase::Skip => {
                    assert!(true);
                }
                TestCase::Ok {
                    description,
                    json,
                    path,
                    remainder,
                    source,
                } => {
                    println!("test {}", &path);
                    let result = list_item_spans(
                        &source,
                        &config,
                        &BlockParent::ListItem,
                        "list-item",
                    )
                    .unwrap();
                    let left_content = (
                        format!("Content: {}", &path),
                        serde_json::from_str::<Block>(&json).unwrap(),
                    );
                    let right_content =
                        (format!("Content: {}", &path), result.1);
                    assert_eq!(left_content, right_content);
                    let left_remainder =
                        (format!("Remainder: {}", &path), remainder);
                    let right_remainder =
                        (format!("Remainder: {}", &path), result.0.to_string());
                    assert_eq!(left_remainder, right_remainder);
                }
                TestCase::Err {
                    description,
                    path,
                    source,
                } => {
                    println!("test {}", &path);
                    let result = list_item_spans(
                        &source,
                        &config,
                        &BlockParent::ListItem,
                        "list-item",
                    );
                    match result {
                        Ok(_) => {
                            println!(
                                "ERROR: Should not have gotten valid response"
                            );
                            assert!(false);
                        }
                        Err(_) => {
                            assert!(true);
                        }
                    }
                }
            }
        }
    }
}
