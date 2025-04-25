#![allow(unused)]
use nom::IResult;
use nom::Parser;
use nom::branch::alt;
use nom::bytes::complete::is_a;
use nom::bytes::complete::is_not;
use nom::bytes::complete::tag;
use nom::character::complete::line_ending;
use nom::character::complete::space0;
use nom::character::complete::space1;
use nom::combinator::eof;
use nom::combinator::not;
use nom::multi::many1;
use nom::sequence::pair;
use nom::sequence::preceded;
use nom::sequence::terminated;

pub fn plain_text_single_line_ending_as_space(
    source: &str,
) -> IResult<&str, &str> {
    let (source, _) =
        ((space0, tag("\n"), not(pair(space0, tag("\n"))))).parse(source)?;
    Ok((source, " "))
}

#[cfg(test)]
mod test {
    use super::*;
    use pretty_assertions::assert_eq;
    use rstest::rstest;

    #[rstest]
    #[case("\n", " ", "")]
    #[case(" \nx", " ", "x")]
    fn plain_text_single_line_ending_as_space_valid_tests(
        #[case] source: &str,
        #[case] got: &str,
        #[case] remainder: &str,
    ) {
        let matcher = (remainder, got);
        let parsed = plain_text_single_line_ending_as_space(source).unwrap();
        assert_eq!(matcher, parsed);
    }

    #[rstest]
    #[case("\n\n", "Two Newlines In A Row")]
    #[case("\n  \n", "Two Newlines Separated By Whitespace")]
    fn plain_text_single_newline_as_space_invalid_tests(
        #[case] source: &str,
        #[case] description: &str,
    ) {
        let parsed = plain_text_single_line_ending_as_space(source);
        if parsed.is_err() {
            assert!(true);
        } else {
            assert!(false, "##### ERROR ##### {}", description);
        }
    }
}
