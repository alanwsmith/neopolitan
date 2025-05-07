use crate::span::Span;
use crate::span::escaped::escaped_span;
use crate::span_metadata::span_metadata;
use crate::span_metadata::strings::escaped_character::escaped_character;
use crate::span_metadata::strings::plain_text_single_line_ending_as_space::plain_text_single_line_ending_as_space;
use crate::span_metadata::strings::plain_text_space1_as_single_space::plain_text_space1_as_single_space;
use crate::span_metadata::strings::plain_text_string_base::plain_text_string_base;
use nom::IResult;
use nom::Parser;
use nom::branch::alt;
use nom::bytes::complete::is_a;
use nom::bytes::complete::tag;
use nom::combinator::opt;
use nom::multi::many1;
use nom::sequence::pair;
use nom::sequence::preceded;

pub fn code_shorthand(source: &str) -> IResult<&str, Span> {
    let (source, spans) = preceded(
        pair(tag("``"), opt(plain_text_space1_as_single_space)),
        many1(alt((escaped_span, code_span_text))),
    )
    .parse(source)?;
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

// TODO: Move this to text_span_in_span and
// pass character to it.

pub fn code_span_text(source: &str) -> IResult<&str, Span> {
    let (source, parts) = many1(alt((
        plain_text_string_base,
        plain_text_space1_as_single_space,
        is_a("%@~*^![]{}<>_#:"),
        plain_text_single_line_ending_as_space,
        escaped_character,
    )))
    .parse(source)?;
    Ok((
        source,
        Span::Text {
            content: parts.join("").trim().to_string(),
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

``alfa``

######

{ 
    "category": "code-shorthand", 
    "kind": "code-shorthand", 
    "attrs": {}, 
    "flags": [], 
    "spans": [
        {
            "category": "text", 
            "content": "alfa",
            "kind": "text"
        }
    ]
}

######

``alfa|bravo``

######

{ 
    "category": "code-shorthand", 
    "kind": "code-shorthand", 
    "attrs": {}, 
    "flags": ["bravo"], 
    "spans": [
        {
            "category": "text", 
            "content": "alfa",
            "kind": "text"
        }
    ]
}

######

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
    // #[ignore]
    // fn code_span_test_runner() {
    //     let cases: Vec<_> = TEST_DATA.split("######").collect();
    //     for case in cases {
    //         let parts: Vec<_> =
    //             case.split("######").map(|c| c.trim()).collect();
    //         let left: Span = serde_json::from_str(parts[1]).unwrap();
    //         let right = code_span(parts[0]).unwrap().1;
    //         assert_eq!(left, right);
    //     }
    // }

    // #[test]
    // fn code_span_single_flag_test() {
    //     let source = "``alfa|bravo``";
    //     let left = Span::Code {
    //         attrs: BTreeMap::new(),
    //         flags: vec!["bravo".to_string()],
    //         kind: "code-shorthand".to_string(),
    //         spans: vec![Span::Text {
    //             content: "alfa".to_string(),
    //             kind: "text".to_string(),
    //         }],
    //     };
    //     let remainder = "";
    //     let right = code_span(source).unwrap();
    //     assert_eq!(left, right.1);
    //     assert_eq!(remainder, right.0);
    // }

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

    //
    //}

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
