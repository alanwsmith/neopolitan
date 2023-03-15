use nom::bytes::complete::tag;
use nom::bytes::complete::take_until1;
use nom::character::complete::multispace0;
use nom::character::complete::not_line_ending;
use nom::combinator::eof;
use nom::multi::many_till;
use nom::IResult;

// pub fn get_attributes(source: &str) -> IResult<&str, HashMap<String, String>> {
pub fn get_attributes(source: &str) -> IResult<&str, Vec<(String, String)>> {
    let (_, attributes) = many_till(attribute_splitter, eof)(source)?;
    Ok(("", attributes.0))
}

pub fn attribute_splitter(source: &str) -> IResult<&str, (String, String)> {
    let (source, _) = multispace0(source)?;
    let (source, _) = tag("-> ")(source)?;
    let (source, key) = take_until1(":")(source)?;
    let (source, _) = tag(":")(source)?;
    let (source, _) = multispace0(source)?;
    let (source, value) = not_line_ending(source)?;

    // dbg!(&key);
    // dbg!(&value);

    Ok((source, (key.to_string(), value.to_string())))
}
