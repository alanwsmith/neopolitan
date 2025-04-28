#![allow(unused)]
// use super::PlainTextSpan;
// use super::SpanV42;
use nom::branch::alt;
use nom::bytes::complete::is_a;
use nom::bytes::complete::is_not;
use nom::bytes::complete::tag;
use nom::character::complete::line_ending;
// use nom::character::complete::multispace1;
use nom::IResult;
use nom::Parser;
use nom::character::complete::space0;
use nom::character::complete::space1;
use nom::combinator::eof;
use nom::combinator::not;
use nom::multi::many1;
use nom::sequence::pair;
use nom::sequence::preceded;
use nom::sequence::terminated;
// use nom::sequence::tuple;

pub fn plain_text_space1_as_single_space(source: &str) -> IResult<&str, &str> {
    let (source, _) = space1.parse(source)?;
    let (source, _) = not(alt((line_ending, eof))).parse(source)?;
    Ok((source, " "))
}

#[cfg(test)]
mod test {
    use super::*;
    use pretty_assertions::assert_eq;
    use rstest::rstest;

    #[rstest]
    #[case(" foxtrot", "foxtrot")]
    #[case("    foxtrot", "foxtrot")]
    fn plain_text_space1_as_single_space_valid_tests(
        #[case] source: &str,
        #[case] remainder: &str,
    ) {
        let matcher = (remainder, " ");
        let parsed = plain_text_space1_as_single_space(source).unwrap();
        assert_eq!(matcher, parsed);
    }

    #[rstest]
    #[case("\n", "Don't Trigger On Newline")]
    #[case(" \n", "Don't Trigger In Front Of Newline")]
    #[case(" ", "Don't Trigger At End Of File")]
    fn plain_text_space1_as_single_space_invalid_tests(
        #[case] source: &str,
        #[case] description: &str,
    ) {
        let parsed = plain_text_space1_as_single_space(source);
        assert!(parsed.is_err(), "{}", description);
    }
}
