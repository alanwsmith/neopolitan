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
        is_not(" \r\n\t\\~`@^*_()[]{}<>-"),
        (space1, not(line_ending)).map(|_| " "),
    )))
    .parse(source)?;
    Ok((
        source,
        Span::Text {
            content: content.join(""),
            kind: "text".to_string()
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
    fn text_in_block_tests() {
        let config = Config::default();
        let file_list = get_file_list(
            &PathBuf::from("src/span/text_in_block/tests"),
            &vec!["neotest".to_string()],
        )
        .unwrap();
        for source_path in file_list {
            match get_test_data(&source_path) {
                TestCase::ExpectingErr {
                    description,
                    path,
                    source,
                } => {
                    dbg!("TODO: Move this to the generic process");
                    assert!(false);
                }
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
                    let result = text_in_block(&source).unwrap();
                    let left_content = (
                        path.clone(),
                        serde_json::from_str::<Span>(&json).unwrap(),
                    );
                    let right_content = (path.clone(), result.1);
                    assert_eq!(left_content, right_content);
                    let left_remainder = (&path, remainder);
                    let right_remainder = (&path, result.0.to_string());
                    assert_eq!(left_remainder, right_remainder);
                }
                TestCase::Err {
                    description,
                    path,
                    source,
                } => {
                    println!("test {}", &path);
                    let result = text_in_block(&source);
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
