use nom::bytes::complete::tag;
use nom::bytes::complete::take_until1;
use nom::character::complete::multispace0;
use nom::character::complete::not_line_ending;
use nom::IResult;
// use std::collections::HashMap;

// pub fn get_attributes(source: &str) -> IResult<&str, HashMap<String, String>> {
pub fn get_attributes(source: &str) -> IResult<&str, Vec<(String, String)>> {
    let mut attributes: Vec<(String, String)> = vec![];

    // let mut attributes = HashMap::new();
    let (source, _) = multispace0(source)?;
    let (source, _) = tag("-> ")(source)?;
    let (source, key) = take_until1(":")(source)?;
    let (source, _) = tag(":")(source)?;
    let (source, _) = multispace0(source)?;
    let (_, value) = not_line_ending(source)?;
    // dbg!(&source);
    // dbg!(&key);
    // attributes.insert(key.to_string(), value.to_string());

    attributes.push((key.to_string(), value.to_string()));

    Ok(("", attributes))
}

// pub fn attribute_splitter(source: &str) -> IResult<&str,
