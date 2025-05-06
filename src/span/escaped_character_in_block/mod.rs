use crate::span::Span;
use nom::Parser;
use nom::branch::alt;
use nom::sequence::preceded;
use nom::{IResult, bytes::tag};

pub fn escaped_character_in_block(source: &str) -> IResult<&str, Span> {
    let (source, character) = alt((
        preceded(tag("\\"), tag("~")),
        preceded(tag("\\"), tag("`")),
        preceded(tag("\\"), tag("@")),
        preceded(tag("\\"), tag("^")),
        preceded(tag("\\"), tag("*")),
        preceded(tag("\\"), tag("_")),
        preceded(tag("\\"), tag("(")),
        preceded(tag("\\"), tag(")")),
        preceded(tag("\\"), tag("[")),
        preceded(tag("\\"), tag("]")),
        preceded(tag("\\"), tag("{")),
        preceded(tag("\\"), tag("}")),
        preceded(tag("\\"), tag("<")),
        preceded(tag("\\"), tag(">")),
    ))
    .parse(source)?;
    Ok((
        source,
        Span::Escaped {
            content: character.to_string(),
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
    fn solo_escaped_character_in_block_tests() {
        let test_file_list = get_file_list(
            &PathBuf::from("src/span/escaped_character_in_block/tests"),
            &vec!["neotest".to_string()],
        )
        .unwrap();
        for source_path in test_file_list {
            match run_span_test_case(&source_path, &escaped_character_in_block)
            {
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
