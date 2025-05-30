// DEPRECATED: Move to escaped_character_in_block, etc...
use crate::span::Span;
use crate::span_metadata::strings::escaped_character::escaped_character;
use nom::IResult;

pub fn escaped_span(source: &str) -> IResult<&str, Span> {
    let (source, content) = escaped_character(source)?;
    Ok((
        source,
        Span::Escaped {
            content: content.to_string(),
            kind: "escaped-span".to_string(),
        },
    ))
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::span::Span;
    use pretty_assertions::assert_eq;
    use rstest::rstest;

    #[rstest]
    #[case("\\|", Span::Escaped{
        content: "|".to_string(),
        kind: "escaped-span".to_string()
    }, "")]
    #[case("\\\\ ", Span::Escaped{
        content: "\\".to_string(),
        kind: "escaped-span".to_string()
    }, " ")]
    fn escaped_span_valid_tests(
        #[case] source: &str,
        #[case] left: Span,
        #[case] remainder: &str,
    ) {
        let right = escaped_span(source).unwrap();
        assert_eq!(left, right.1);
        assert_eq!(remainder, right.0);
    }
}
