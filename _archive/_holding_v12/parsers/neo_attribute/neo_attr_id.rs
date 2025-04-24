use nom::bytes::complete::is_not;
use nom::character::complete::alpha1;
use nom::IResult;

pub fn neo_attr_id(source: &str) -> IResult<&str, String> {
    let (source, value) = is_not(">|")(source)?;
    Ok((source, value.to_string()))
}
