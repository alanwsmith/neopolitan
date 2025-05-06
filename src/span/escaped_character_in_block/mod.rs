use crate::span::Span;
use nom::Parser;
use nom::branch::alt;
use nom::sequence::preceded;
use nom::{IResult, bytes::complete::tag};

// NOTE: The closing ``)``, ``]``, ``}``, and
// ``>`` aren't strictly necessary since they
// only need to be escaped inside a shorthand
// or tag. Provided the escapes regardless
// so it works regardless.
//
// NOTE: Same goes for the "|" pipe character.
// It doesn't need to be escaped in block content
// but this makes sure it works if someone does.
// (It would be friction if it didn't cause
// you'd have to think about it)
//
// NOTE: The ``-`` is so you can start a new
// list-item-spans block with a ``-`` inside
// a ``list-item`` block without starting
// a new ``list-item-spans``.

// The number character and hashtag escapes
// are so you can start a text block inside
// a num-list with those characters without
// starting a new item.
//

pub fn escaped_character_in_block(source: &str) -> IResult<&str, Span> {
    let (source, character) = alt((
        alt((
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
            preceded(tag("\\"), tag("|")),
            preceded(tag("\\"), tag("\\")),
        )),
        alt((
            preceded(tag("\\"), tag("-")),
            preceded(tag("\\"), tag("1")),
            preceded(tag("\\"), tag("2")),
            preceded(tag("\\"), tag("3")),
            preceded(tag("\\"), tag("4")),
            preceded(tag("\\"), tag("5")),
            preceded(tag("\\"), tag("6")),
            preceded(tag("\\"), tag("7")),
            preceded(tag("\\"), tag("8")),
            preceded(tag("\\"), tag("9")),
            preceded(tag("\\"), tag("0")),
            preceded(tag("\\"), tag("#")),
        )),
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
    fn escaped_character_in_block_tests() {
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
