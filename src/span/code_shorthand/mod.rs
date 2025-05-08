#![allow(unused)]
use super::single_line_ending::single_line_ending;
use crate::span::Span;
use crate::span::escaped::escaped_span;
use crate::span_metadata::span_metadata;
use crate::span_metadata::strings::escaped_character::escaped_character;
// use crate::span_metadata::strings::plain_text_single_line_ending_as_space::plain_text_single_line_ending_as_space;
// use crate::span_metadata::strings::plain_text_space1_as_single_space::plain_text_space1_as_single_space;
// use crate::span_metadata::strings::plain_text_string_base::plain_text_string_base;
use nom::IResult;
use nom::Parser;
use nom::branch::alt;
use nom::bytes::complete::is_a;
use nom::bytes::complete::is_not;
use nom::bytes::complete::tag;
use nom::character::complete::line_ending;
use nom::character::complete::multispace0;
use nom::character::complete::space0;
use nom::character::complete::space1;
use nom::combinator::complete;
use nom::combinator::opt;
use nom::combinator::peek;
use nom::multi::many1;
use nom::sequence::pair;
use nom::sequence::preceded;
use std::collections::BTreeMap;

pub fn code_shorthand(source: &str) -> IResult<&str, Span> {
    let (source, _) = (tag("``"), space0).parse(source)?;
    let (source, _) = opt(single_line_ending).parse(source)?;
    let (source, spans) = many1(alt((code_span_text,))).parse(source)?;
    let (source, metadata) = span_metadata(source, "`")?;
    let (source, _) = tag("``").parse(source)?;
    Ok((
        source,
        Span::Code {
            attrs: metadata.attrs,
            flags: metadata.flags,
            kind: "code-shorthand".to_string(),
            spans,
        },
    ))
}

pub fn code_span_text(source: &str) -> IResult<&str, Span> {
    let (source, parts) = many1(alt((
        (space1, peek(tag("``"))).map(|_| ""),
        space1.map(|_| " "),
        is_not(" \r\n\t|`"),
        (line_ending, peek(tag("``"))).map(|_| ""),
        (line_ending, peek(tag("|"))).map(|_| ""),
        single_line_ending.map(|_| " "),
    )))
    .parse(source)?;
    Ok((
        source,
        Span::Text {
            content: parts.join(""),
            kind: "text".to_string(),
        },
    ))
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::span::Span;
    use pretty_assertions::assert_eq;
    // use rstest::rstest;
    use crate::helpers::*;
    use std::collections::BTreeMap;
    use std::path::PathBuf;

    const TEST_DATA: &str = r#"

``alfa|bravo: charlie|delta``

######

{ 
    "category": "code-shorthand", 
    "kind": "code-shorthand", 
    "attrs": {
        "bravo": [
            {
                "category": "text", 
                "content": "charlie",
                "kind": "text"
            }
        ]
    }, 
    "flags": ["delta"], 
    "spans": [
        {
            "category": "text", 
            "content": "alfa",
            "kind": "text"
        }
    ]
}


"#;

    // #[test]
    // fn code_span_newline_test() {
    //     let source = "``\nalfa\n|\nbravo:\ncharlie\n|\ndelta\n`` ping";
    //     let mut attrs = BTreeMap::new();
    //     attrs.insert(
    //         "bravo".to_string(),
    //         vec![Span::Text {
    //             content: "charlie ".to_string(),
    //             kind: "text".to_string(),
    //         }],
    //     );
    //     let flags = vec!["delta".to_string()];
    //     let left = Span::Code {
    //         attrs,
    //         flags,
    //         kind: "code-shorthand".to_string(),
    //         spans: vec![Span::Text {
    //             content: "alfa".to_string(),
    //             kind: "text".to_string(),
    //         }],
    //     };
    //     let remainder = " ping";
    //     let right = code_span(source).unwrap();
    //     assert_eq!(left, right.1);
    //     assert_eq!(remainder, right.0);
    // }

    // #[test]
    // fn code_span_newline_test_2() {
    //     let source = "``\nalfa\n|\nbravo:\ncharlie \n echo\n|\ndelta\n`` ping";
    //     let mut attrs = BTreeMap::new();
    //     attrs.insert(
    //         "bravo".to_string(),
    //         vec![Span::Text {
    //             content: "charlie echo ".to_string(),
    //             kind: "text".to_string(),
    //         }],
    //     );
    //     let flags = vec!["delta".to_string()];
    //     let left = Span::Code {
    //         attrs,
    //         flags,
    //         kind: "code-shorthand".to_string(),
    //         spans: vec![Span::Text {
    //             content: "alfa".to_string(),
    //             kind: "text".to_string(),
    //         }],
    //     };
    //     let remainder = " ping";
    //     let right = code_span(source).unwrap();
    //     assert_eq!(left, right.1);
    //     assert_eq!(remainder, right.0);
    // }

    #[test]
    fn solo_code_shorthand_tests() {
        let source_dir = &PathBuf::from("src/span/code_shorthand/tests");
        let test_file_list =
            get_file_list(&source_dir, &vec!["neotest".to_string()]).unwrap();
        for source_path in test_file_list {
            println!("test {}", &source_path.display());
            match run_span_test_case(&source_path, &code_shorthand) {
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
