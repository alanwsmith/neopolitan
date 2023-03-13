// use nom::bytes::complete::tag;
// use nom::bytes::complete::take_until1;
// use nom::character::complete::multispace0;
// use nom::IResult;

// pub fn get_attribute(source: &str) -> IResult<&str, (String, String)> {
//     let (source, _) = multispace0(source)?;
//     let (source, _) = tag("-> ")(source)?;
//     let (source, key) = take_until1(": ")(source)?;
//     let (source, _) = tag(":")(source)?;
//     let (source, _) = multispace0(source)?;
//     let (source, value) = take_until1("\n")(source)?;
//     let (source, _) = multispace0(source)?;
//     Ok((source, (key.trim().to_string(), value.trim().to_string())))
// }
