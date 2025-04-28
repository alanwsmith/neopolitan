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

pub fn plain_text_string_base(source: &str) -> IResult<&str, &str> {
    let (source, result) = alt((
        is_not(" \t\r\n`~!@#%^*[]{}<>_|:\\"),
        terminated(tag("`"), not(tag("`"))),
        terminated(tag("~"), not(tag("~"))),
        terminated(tag("!"), not(tag("!"))),
        terminated(tag("@"), not(tag("@"))),
        terminated(tag("#"), not(tag("#"))),
        terminated(tag("%"), not(tag("%"))),
        terminated(tag("^"), not(tag("^"))),
        terminated(tag("*"), not(tag("*"))),
        terminated(tag("["), not(tag("["))),
        terminated(tag("]"), not(tag("]"))),
        terminated(tag("{"), not(tag("{"))),
        terminated(tag("}"), not(tag("}"))),
        terminated(tag("<"), not(tag("<"))),
        terminated(tag(">"), not(tag(">"))),
        terminated(tag("_"), not(tag("_"))),
    ))
    .parse(source)?;
    Ok((source, result))
}

#[cfg(test)]
mod test {
    use super::*;
    use pretty_assertions::assert_eq;
    use rstest::rstest;
    #[rstest]
    #[case("alfa", "alfa", "")]
    #[case("alfa bravo", "alfa", " bravo")]
    #[case("alfa\nbravo", "alfa", "\nbravo")]
    #[case("`bravo", "`", "bravo")]
    #[case("~bravo", "~", "bravo")]
    #[case("!bravo", "!", "bravo")]
    #[case("#bravo", "#", "bravo")]
    #[case("^bravo", "^", "bravo")]
    #[case("*bravo", "*", "bravo")]
    #[case("[bravo", "[", "bravo")]
    #[case("]bravo", "]", "bravo")]
    #[case("{bravo", "{", "bravo")]
    #[case("}bravo", "}", "bravo")]
    #[case("<bravo", "<", "bravo")]
    #[case(">bravo", ">", "bravo")]
    #[case("_bravo", "_", "bravo")]
    fn plain_text_string_base_valid_tests(
        #[case] source: &str,
        #[case] got: &str,
        #[case] remainder: &str,
    ) {
        let matcher = (remainder, got);
        let parsed = plain_text_string_base(source).unwrap();
        assert_eq!(matcher, parsed);
    }
}
