#![allow(unused)]
use crate::span::Span;
use nom::Parser;
use nom::branch::alt;
use nom::character::complete::{line_ending, space0};
use nom::combinator::eof;
use nom::combinator::not;
use nom::multi::many0;
use nom::sequence::terminated;
use nom::{IResult, bytes::complete::tag};

pub fn empty_line_or_lines_after_line_ending_or_eof(
    source: &str,
) -> IResult<&str, Span> {
    let (source, _) =
        alt(((space0, line_ending, space0, line_ending).map(|_| ""), eof))
            .parse(source)?;
    let (source, _) = many0((space0, line_ending)).parse(source)?;
    Ok((
        source,
        Span::EmptyLineOrLines {
            kind: "empty-line-or-lines".to_string(),
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
    fn solo_empty_lines_after_line_ending_tests() {
        let source_dir = &PathBuf::from(
            "src/span/empty_line_or_lines_after_line_ending_or_eof/tests",
        );
        let test_file_list =
            get_file_list(&source_dir, &vec!["neotest".to_string()]).unwrap();
        for source_path in test_file_list {
            println!("test {}", &source_path.display());
            match run_span_test_case(
                &source_path,
                &empty_line_or_lines_after_line_ending_or_eof,
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
                TestSpanPayload::ExpectedError => {
                    assert!(true);
                }
                TestSpanPayload::ShouldHaveErroredButDidNot => {
                    dbg!(
                        "###### This should have failed, but it passed ######"
                    );
                    assert!(false);
                }
                TestSpanPayload::Skip => {
                    dbg!("###### Test skipped here ######");
                    assert!(true);
                }
            }
        }
    }

    //
}
