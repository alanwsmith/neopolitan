use crate::span::Span;
use crate::span_metadata::strings::multiple_pipes::multiple_pipes;
use crate::span_metadata::strings::plain_text_any_colons::plain_text_any_colons;
use crate::span_metadata::strings::plain_text_single_line_ending_as_space::plain_text_single_line_ending_as_space;
use crate::span_metadata::strings::plain_text_space1_as_single_space::plain_text_space1_as_single_space;
use crate::span_metadata::strings::plain_text_string_base::plain_text_string_base;
use crate::span_metadata::strings::single_pipe::single_pipe;
use nom::IResult;
use nom::Parser;
use nom::branch::alt;
use nom::multi::many1;

pub fn text_span_in_block<'a>(source: &'a str) -> IResult<&'a str, Span> {
    let (source, results) = many1(alt((
        plain_text_string_base,
        plain_text_space1_as_single_space,
        plain_text_single_line_ending_as_space,
        plain_text_any_colons,
        single_pipe,
        multiple_pipes,
    )))
    .parse(source)?;
    Ok((
        source,
        Span::Text {
            content: results.join("").to_string(),
        },
    ))
}

#[cfg(test)]
mod test {
    use super::*;
    use pretty_assertions::assert_eq;
    use rstest::rstest;

    #[rstest]
    #[case("alfa", Span::Text{ content: "alfa".to_string()}, "")]
    #[case("alfa bravo", Span::Text{ content: "alfa bravo".to_string()}, "")]
    #[case("alfa \"bravo\"", Span::Text{ content: "alfa \"bravo\"".to_string()}, "")]
    #[case("alfa` bravo", Span::Text{ content: "alfa` bravo".to_string()}, "")]
    #[case("alfa~ bravo", Span::Text{ content: "alfa~ bravo".to_string()}, "")]
    #[case("alfa\nbravo", Span::Text{ content: "alfa bravo".to_string()}, "")]
    #[case("alfa \nbravo", Span::Text{ content: "alfa bravo".to_string()}, "")]
    #[case("alfa\n\nbravo", Span::Text{ content: "alfa".to_string()}, "\n\nbravo")]
    #[case("https://www.example.com/", Span::Text{ content: "https://www.example.com/".to_string()}, "")]
    #[case("alfa bravo -\n- charlie delta", Span::Text{ content: "alfa bravo - - charlie delta".to_string()}, "")]
    #[case("alfa^^1^^", Span::Text{ content: "alfa".to_string()}, "^^1^^")]
    #[case("alfa <<span|ping>>", Span::Text{ content: "alfa ".to_string()}, "<<span|ping>>")]
    #[case("alfa\\<<", Span::Text{ content: "alfa".to_string()}, "\\<<")]
    #[case("alfa|bravo", Span::Text{ content: "alfa|bravo".to_string()}, "")]
    #[case("alfa||bravo", Span::Text{ content: "alfa||bravo".to_string()}, "")]
    fn text_span_valid_tests(
        #[case] source: &str,
        #[case] left: Span,
        #[case] remainder: &str,
    ) {
        let right = text_span_in_block(source).unwrap();
        assert_eq!(left, right.1);
        assert_eq!(remainder, right.0);
    }

    #[rstest]
    #[case("``alfa")]
    #[case("<<alfa")]
    fn text_span_invalid_tests(#[case] source: &str) {
        let result = text_span_in_block(source);
        match result {
            Ok(_) => {
                dbg!(result.unwrap());
                assert!(false)
            }
            Err(_) => assert!(true),
        }
    }
}
