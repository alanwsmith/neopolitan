use nom::IResult;
use nom::Parser;
use nom::branch::alt;
use nom::character::complete::line_ending;
use nom::character::complete::space0;
use nom::combinator::eof;
use nom::sequence::pair;

// NOTE: This is for chomping empty
// space at the end of lines or a file

pub fn space0_line_ending_or_eof(source: &str) -> IResult<&str, &str> {
    let (source, _) =
        alt((pair(space0, line_ending), pair(space0, eof))).parse(source)?;
    Ok((source, ""))
}

#[cfg(test)]
mod test {
    use super::*;
    use pretty_assertions::assert_eq;
    use rstest::rstest;

    #[rstest]
    #[case("", "", "")]
    #[case(" ", "", "")]
    #[case("\n", "", "")]
    #[case(" \n", "", "")]
    #[case(" \nx", "", "x")]
    fn space0_line_ending_or_eof_valid_tests(
        #[case] source: &str,
        #[case] got: &str,
        #[case] remainder: &str,
    ) {
        let matcher = (remainder, got);
        let parsed = space0_line_ending_or_eof(source).unwrap();
        assert_eq!(matcher, parsed);
    }
}
