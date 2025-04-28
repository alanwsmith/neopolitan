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

pub fn plain_text_any_pipes(source: &str) -> IResult<&str, &str> {
    let (source, result) = is_a("|").parse(source)?;
    Ok((source, result))
}

#[cfg(test)]
mod test {
    use super::*;
    use pretty_assertions::assert_eq;
    use rstest::rstest;

    #[rstest]
    #[case("|", "|", "")]
    #[case("||", "||", "")]
    #[case("|||", "|||", "")]
    #[case("|x|", "|", "x|")]
    fn plain_text_any_pipes_valid_tests(
        #[case] source: &str,
        #[case] got: &str,
        #[case] remainder: &str,
    ) {
        let matcher = (remainder, got);
        let parsed = plain_text_any_pipes(source).unwrap();
        assert_eq!(matcher, parsed);
    }
}
