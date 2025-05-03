use crate::span::Span;
use crate::span_metadata::strings::plain_text_any_colons::plain_text_any_colons;
use crate::span_metadata::strings::plain_text_single_line_ending_as_space::plain_text_single_line_ending_as_space;
use crate::span_metadata::strings::plain_text_space1_as_single_space::plain_text_space1_as_single_space;
use crate::span_metadata::strings::plain_text_string_base::plain_text_string_base;
use nom::IResult;
use nom::Parser;
use nom::branch::alt;
use nom::character::complete::line_ending;
use nom::character::complete::multispace0;
use nom::character::complete::space0;
use nom::combinator::not;
use nom::multi::many1;

// NOTE: Having empty lines in an
// attribute is not allowed. (as in an
// empty line between the different
// parts of the attribute in the same
// way empty lines break paragraphs.

// NOTE: Trailing space is required
// to be left in since there can be other spans
// after it.
// TODO: Figure out a way to chomp
// just the last one?

pub fn text_span_in_span_attr(source: &str) -> IResult<&str, Span> {
    let (source, results) = many1(alt((
        plain_text_string_base,
        plain_text_space1_as_single_space,
        plain_text_single_line_ending_as_space,
        plain_text_any_colons,
    )))
    .parse(source)?;
    let (source, _) = not((line_ending, space0, line_ending)).parse(source)?;
    let (source, _) = multispace0(source)?;
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
    #[case("https://www.example.com/", Span::Text{ content: "https://www.example.com/".to_string()}, "")]
    #[case("alfa bravo -\n- charlie delta", Span::Text{ content: "alfa bravo - - charlie delta".to_string()}, "")]
    #[case("alfa^^1^^", Span::Text{ content: "alfa".to_string()}, "^^1^^")]
    #[case("alfa\\<<", Span::Text{ content: "alfa".to_string()}, "\\<<")]
    // TODO: Make escaped version of this
    // #[case("alfa|bravo", Span::Text{ content: "alfa|bravo".to_string()}, "")]
    fn text_span_in_span_attr_valid_tests(
        #[case] source: &str,
        #[case] left: Span,
        #[case] remainder: &str,
    ) {
        let right = text_span_in_span_attr(source).unwrap();
        assert_eq!(left, right.1);
        assert_eq!(remainder, right.0);
    }

    #[test]
    fn text_span_in_span_attr_whitespace_test() {
        let source = "alfa    bravo \n   ";
        let left = Span::Text {
            content: "alfa bravo ".to_string(),
        };
        let remainder = "";
        let right = text_span_in_span_attr(source).unwrap();
        assert_eq!(remainder, right.0);
        assert_eq!(left, right.1);
    }

    #[test]
    fn text_span_in_span_attr_chomp_leading_whitespace_on_new_lines() {
        let source = "alfa  \n  bravo";
        let left = Span::Text {
            content: "alfa bravo".to_string(),
        };
        let remainder = "";
        let right = text_span_in_span_attr(source).unwrap();
        assert_eq!(remainder, right.0);
        assert_eq!(left, right.1);
    }

    #[test]
    fn text_span_in_span_attr_no_empty_lines() {
        let source = "alfa\n\nbravo";
        match text_span_in_span_attr(source) {
            Ok(result) => {
                dbg!(result);
                assert!(false);
            }
            Err(_) => {
                assert!(true);
            }
        }
    }

    #[test]
    fn text_span_in_span_attr_trailing_space_is_necessary() {
        let source = "alfa <<span|ping>>";
        let left = Span::Text {
            content: "alfa ".to_string(),
        };
        let remainder = "<<span|ping>>";
        let right = text_span_in_span_attr(source).unwrap();
        assert_eq!(remainder, right.0);
        assert_eq!(left, right.1);
    }
}
