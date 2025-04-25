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
use nom::sequence::tuple;

pub fn plain_text_single_line_ending_as_space(
    source: &str,
) -> IResult<&str, &str> {
    let (source, _) = tuple((space0, tag("\n"), not(pair(space0, tag("\n")))))
        .parse(source)?;
    Ok((source, " "))
}
