use nom::IResult;
use nom::Parser;
use nom::branch::alt;
use nom::character::complete::line_ending;
use nom::character::complete::space0;
use nom::combinator::eof;
use nom::sequence::pair;

pub fn space0_line_ending_or_eof(source: &str) -> IResult<&str, &str> {
    let (source, _) =
        alt((pair(space0, line_ending), pair(space0, eof))).parse(source)?;
    Ok((source, ""))
}

//#[cfg(test)]
//mod test {
//    use super::*;
//    use pretty_assertions::assert_eq;
//    //
//}
