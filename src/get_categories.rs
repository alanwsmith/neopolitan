use nom::bytes::complete::tag;
use nom::character::complete::multispace0;
use nom::character::complete::not_line_ending;
use nom::combinator::eof;
use nom::multi::many_till;
use nom::IResult;

pub fn get_categories(source: &str) -> IResult<&str, Vec<String>> {
    let (_, categories) = many_till(category_splitter, eof)(source)?;
    Ok(("", categories.0))
}

pub fn category_splitter(source: &str) -> IResult<&str, String> {
    let (source, _) = multispace0(source)?;
    let (source, _) = tag("- ")(source)?;
    let (source, value) = not_line_ending(source)?;
    let (source, _) = multispace0(source)?;
    Ok((source, value.trim().to_string()))
}
