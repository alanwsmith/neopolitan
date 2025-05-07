// DEPRECATED: Remove next time you see this
use crate::span::Span;
use nom::IResult;
use nom::Parser;
use nom::character::complete::space1;

pub fn space1_as_single_space(source: &str) -> IResult<&str, Span> {
    let (source, _) = space1.parse(source)?;
    Ok((
        source,
        Span::Space {
            kind: "space".to_string(),
        },
    ))
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::helpers::*;
    use std::path::PathBuf;

    #[test]
    fn solo_space_span_tests() {
        let source_dir =
            &PathBuf::from("src/span/space1_as_single_space/tests");
        let test_file_list =
            get_file_list(&source_dir, &vec!["neotest".to_string()]).unwrap();
        for source_path in test_file_list {
            println!("test {}", &source_path.display());
            match run_span_test_case(&source_path, &space1_as_single_space) {
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
