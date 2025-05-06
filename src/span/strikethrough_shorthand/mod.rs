use std::collections::BTreeMap;

use crate::span::Span;
use nom::Parser;
use nom::branch::alt;
use nom::bytes::complete::is_not;
use nom::character::complete::{line_ending, space0};
use nom::combinator::eof;
use nom::combinator::not;
use nom::multi::many0;
use nom::sequence::terminated;
use nom::{IResult, bytes::complete::tag};

pub fn strikethrough_shorthand(source: &str) -> IResult<&str, Span> {
    let (source, _) = tag("~~").parse(source)?;
    let (source, spans) = many0(strikethrough_shorthand_text).parse(source)?;
    let (source, _) = tag("~~").parse(source)?;
    Ok((
        source,
        Span::StrikethroughShorthand {
            attrs: BTreeMap::new(),
            flags: vec![],
            kind: "strikethrough-shorthand".to_string(),
            spans,
        },
    ))
}

pub fn strikethrough_shorthand_text(source: &str) -> IResult<&str, Span> {
    let (source, content) = is_not("~").parse(source)?;
    Ok((
        source,
        Span::Text {
            content: content.to_string(),
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
    fn solo_strikethrough_shorthand_tests() {
        let source_dir =
            &PathBuf::from("src/span/strikethrough_shorthand/tests");
        let test_file_list =
            get_file_list(&source_dir, &vec!["neotest".to_string()]).unwrap();
        for source_path in test_file_list {
            println!("test {}", &source_path.display());
            match run_span_test_case(&source_path, &strikethrough_shorthand) {
                TestSpanPayload::Ok {
                    left_content,
                    right_content,
                    left_remainder,
                    right_remainder,
                } => {
                    assert_eq!(left_content, right_content);
                    assert_eq!(left_remainder, right_remainder);
                }
                TestSpanPayload::UnexpectedError => {
                    dbg!(
                        "###### This should not have failed, but it did ######"
                    );
                    assert!(false);
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
