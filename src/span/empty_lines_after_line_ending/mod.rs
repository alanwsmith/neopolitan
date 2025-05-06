use crate::span::Span;
use nom::Parser;
use nom::branch::alt;
use nom::character::complete::{line_ending, space0};
use nom::combinator::not;
use nom::sequence::terminated;
use nom::{IResult, bytes::complete::tag};

pub fn empty_line_after_line_ending(source: &str) -> IResult<&str, Span> {
    let (source, _) = (space0, line_ending, line_ending).parse(source)?;
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
        let source_dir =
            &PathBuf::from("src/span/empty_lines_after_line_ending/tests");
        let test_file_list =
            get_file_list(&source_dir, &vec!["neotest".to_string()]).unwrap();
        for source_path in test_file_list {
            match run_span_test_case(
                &source_path,
                &empty_line_after_line_ending,
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
