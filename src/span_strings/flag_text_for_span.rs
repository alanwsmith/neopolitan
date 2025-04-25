#![allow(unused)]
use nom::IResult;
use nom::Parser;
use nom::branch::alt;
use nom::bytes::complete::is_a;
use nom::bytes::complete::is_not;
use nom::bytes::complete::tag;
use nom::character::complete::alphanumeric1;
use nom::character::complete::line_ending;
use nom::character::complete::multispace0;
use nom::character::complete::multispace1;
use nom::character::complete::space0;
use nom::character::complete::space1;
use nom::combinator::eof;
use nom::combinator::not;
use nom::multi::many1;
use nom::sequence::pair;
use nom::sequence::preceded;
use nom::sequence::terminated;

// THIS IS PROBABLY DEPRECATED

// NOTE: Flags do not accept escape characters.
// Use an attribute if you need on.

pub fn flag_text_for_span(source: &str) -> IResult<&str, String> {
    let (source, _) = multispace0(source)?;
    let (source, content) = is_not(" ").parse(source)?;

    // let (source, parts) = many1(alt((
    //     is_not(": \n\t\\"),
    //     pair(tag(":"), not(alt((multispace1, eof)))).map(|x| x.0),
    // )))
    // .parse(source)?;
    // let (source, _) =
    // alt((pair(space0, line_ending), pair(space0, eof))).parse(source)?;
    Ok((source, content.to_string()))
}

#[cfg(test)]
mod test {
    use super::*;
    use pretty_assertions::assert_eq;
    use rstest::rstest;

    #[rstest]
    #[case("alfa", "alfa", "")]
    #[case("underscores_are_okay", "underscores_are_okay", "")]
    #[case("dashes-are-okay", "dashes-are-okay", "")]
    #[case("  leading_spaces_are_okay", "leading_spaces_are_okay", "")]
    // #[case("bravo-charlie", "bravo-charlie", "")]
    // #[case("https://www.example.com/", "https://www.example.com/", "")]
    // #[case("trailing_spaces_are_okay   ", "trailing_spaces_are_okay", "")]
    // #[case("split |", "split", "|")]
    // #[case("these_characters_are_okay:!@#$%^&*()[]<>|-")]
    fn flat_text_valid_tests(
        #[case] source: &str,
        #[case] got: String,
        #[case] remainder: &str,
    ) {
        let matcher = (remainder, got);
        let parsed = flag_text_for_span(source).unwrap();
        assert_eq!(matcher, parsed);
    }

    // #[rstest]
    // #[case("no: attrs")]
    // fn flat_text_invalid_tests(#[case] source: &str) {
    //     match flag_text_for_span(source) {
    //         Ok(_) => assert!(false),
    //         Err(_) => assert!(true),
    //     }
    // }

    //
}
