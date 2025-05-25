use crate::span::Span;
use nom::Parser;
use nom::branch::alt;
use nom::combinator::not;
use nom::sequence::terminated;
use nom::{IResult, bytes::complete::tag};

pub fn single_character_allowed_in_block(source: &str) -> IResult<&str, Span> {
    let (source, character) = alt((
        terminated(tag("~"), not(tag("~"))),
        terminated(tag("`"), not(tag("`"))),
        terminated(tag("@"), not(tag("@"))),
        terminated(tag("^"), not(tag("^"))),
        terminated(tag("*"), not(tag("*"))),
        terminated(tag("_"), not(tag("_"))),
        terminated(tag("|"), not(tag("|"))),
        terminated(tag(")"), not(tag(")"))),
        terminated(tag("("), not(tag("("))),
        terminated(tag("["), not(tag("["))),
        terminated(tag("]"), not(tag("]"))),
        terminated(tag("{"), not(tag("{"))),
        terminated(tag("}"), not(tag("}"))),
        terminated(tag("<"), not(tag("<"))),
        terminated(tag(">"), not(tag(">"))),
    ))
    .parse(source)?;
    Ok((
        source,
        Span::Text {
            content: character.to_string(),
            kind: "text".to_string(),
        },
    ))
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::helpers::*;
    use pretty_assertions::assert_eq;
    use std::path::PathBuf;

    #[test]
    fn single_character_allowed_in_block_tests() {
        let source_dir =
            &PathBuf::from("src/span/single_character_allowed_in_block/tests");
        let test_file_list =
            get_file_list(&source_dir, &vec!["neotest".to_string()]).unwrap();
        for source_path in test_file_list {
            match run_span_test_case(
                &source_path,
                &single_character_allowed_in_block,
            ) {
                TestSpanPayload::Ok {
                    left_content,
                    right_content,
                    left_remainder,
                    right_remainder,
                } => {
                    assert_eq!(left_content, right_content);
                    assert_eq!(left_remainder, right_remainder);
                }
                _ => {
                    assert!(false);
                }
            }
        }
    }
}
