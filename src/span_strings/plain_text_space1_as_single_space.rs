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
