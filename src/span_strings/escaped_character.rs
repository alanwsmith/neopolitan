use nom::IResult;
use nom::Parser;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::sequence::preceded;

pub fn escaped_character(source: &str) -> IResult<&str, &str> {
    let (source, result) = alt((
        preceded(tag("\\"), tag(":")),
        preceded(tag("\\"), tag("|")),
        preceded(tag("\\"), tag("`")),
        preceded(tag("\\"), tag("~")),
        preceded(tag("\\"), tag("!")),
        preceded(tag("\\"), tag("#")),
        preceded(tag("\\"), tag("%")),
        preceded(tag("\\"), tag("^")),
        preceded(tag("\\"), tag("*")),
        preceded(tag("\\"), tag("[")),
        preceded(tag("\\"), tag("]")),
        preceded(tag("\\"), tag("{")),
        preceded(tag("\\"), tag("}")),
        preceded(tag("\\"), tag("<")),
        preceded(tag("\\"), tag(">")),
        preceded(tag("\\"), tag("_")),
        preceded(tag("\\"), tag("\\")),
    ))
    .parse(source)?;
    Ok((source, result))
}
