use crate::shorthand_span::shorthand_span;
use crate::span::Span;
use crate::spans::text_span_in_span_attr::text_span_in_span_attr;
use nom::IResult;
use nom::Parser;
use nom::branch::alt;

pub fn span_in_span_attr<'a>(source: &'a str) -> IResult<&'a str, Span> {
    let (source, span) =
        alt((text_span_in_span_attr, shorthand_span)).parse(source)?;
    Ok((source, span))
}

// NOTE: Having empty lines in an
// attribute is not allowed.

// NOTE: Trailing space is required
// since there can be other spans
// after it.
// TODO: Figure out a way to chomp
// just the last one.

#[cfg(test)]
mod test {

    // use super::*;
    // use pretty_assertions::assert_eq;
    // use rstest::rstest;

    // #[rstest]
    // #[case("alfa", Span::Text{ content: "alfa".to_string()}, "")]
    // #[case("alfa bravo", Span::Text{ content: "alfa bravo".to_string()}, "")]
    // #[case("alfa \"bravo\"", Span::Text{ content: "alfa \"bravo\"".to_string()}, "")]
    // #[case("alfa` bravo", Span::Text{ content: "alfa` bravo".to_string()}, "")]
    // #[case("alfa~ bravo", Span::Text{ content: "alfa~ bravo".to_string()}, "")]
    // #[case("alfa\nbravo", Span::Text{ content: "alfa bravo".to_string()}, "")]
    // #[case("alfa \nbravo", Span::Text{ content: "alfa bravo".to_string()}, "")]
    // #[case("https://www.example.com/", Span::Text{ content: "https://www.example.com/".to_string()}, "")]
    // #[case("alfa bravo -\n- charlie delta", Span::Text{ content: "alfa bravo - - charlie delta".to_string()}, "")]
    // #[case("alfa^^1^^", Span::Text{ content: "alfa".to_string()}, "^^1^^")]
    // #[case("alfa\\<<", Span::Text{ content: "alfa".to_string()}, "\\<<")]
    // // TODO: Make escaped version of this
    // // #[case("alfa|bravo", Span::Text{ content: "alfa|bravo".to_string()}, "")]
    // fn text_in_span_attr_valid_tests(
    //     #[case] source: &str,
    //     #[case] left: Span,
    //     #[case] remainder: &str,
    // ) {
    //     let right = text_in_span_attr(source).unwrap();
    //     assert_eq!(left, right.1);
    //     assert_eq!(remainder, right.0);
    // }

    // #[test]
    // fn text_in_span_attr_whitespace_test() {
    //     let source = "alfa    bravo \n   ";
    //     let left = Span::Text {
    //         content: "alfa bravo ".to_string(),
    //     };
    //     let remainder = "";
    //     let right = text_in_span_attr(source).unwrap();
    //     assert_eq!(remainder, right.0);
    //     assert_eq!(left, right.1);
    // }

    // #[test]
    // fn solo_text_in_span_attr_chomp_leading_whitespace_on_new_lines() {
    //     let source = "alfa  \n  bravo";
    //     let left = Span::Text {
    //         content: "alfa bravo".to_string(),
    //     };
    //     let remainder = "";
    //     let right = text_in_span_attr(source).unwrap();
    //     assert_eq!(remainder, right.0);
    //     assert_eq!(left, right.1);
    // }

    // #[test]
    // fn text_in_span_attr_no_empty_lines() {
    //     let source = "alfa\n\nbravo";
    //     match text_in_span_attr(source) {
    //         Ok(result) => {
    //             dbg!(result);
    //             assert!(false);
    //         }
    //         Err(_) => {
    //             assert!(true);
    //         }
    //     }
    // }

    // #[test]
    // fn text_in_span_attr_trailing_space_is_necessary() {
    //     let source = "alfa <<span|ping>>";
    //     let left = Span::Text {
    //         content: "alfa ".to_string(),
    //     };
    //     let remainder = "<<span|ping>>";
    //     let right = text_in_span_attr(source).unwrap();
    //     assert_eq!(remainder, right.0);
    //     assert_eq!(left, right.1);
    // }

    //
}
