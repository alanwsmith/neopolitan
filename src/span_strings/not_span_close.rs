use nom::IResult;
use nom::Parser;
use nom::bytes::complete::is_a;
use nom::bytes::complete::tag;
use nom::combinator::not;
use nom::combinator::recognize;
use nom::sequence::preceded;

pub fn not_span_close<'a>(
    source: &'a str,
    character: &'a str,
) -> IResult<&'a str, &'a str> {
    let (source, result) = recognize(preceded(
        not((tag(character), tag(character))),
        is_a("`~!@#$%^&*()<>[]{}"),
    ))
    .parse(source)?;
    Ok((source, result))
}
