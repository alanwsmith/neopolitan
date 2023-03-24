use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::bytes::complete::take_until;
use nom::character::complete::multispace0;
use nom::combinator::rest;
use nom::multi::many0;
use nom::sequence::tuple;
use nom::IResult;

// this is the one for text attributes
// the other one is for attributes for
// sections: TODO: Rename the other
// one to `section_attributes``
//

pub fn tag_attributes(
    source: &str,
) -> IResult<&str, Option<Vec<(Option<String>, Option<String>)>>> {
    let payload: Vec<(Option<String>, Option<String>)> = vec![];
    let (remainder, mut parts) = many0(part)(source)?;
    parts.push(remainder);
    if parts.len() == 1 {
        Ok((parts[0], None))
    } else {
        let response: Vec<(Option<String>, Option<String>)> = parts
            .iter()
            .map(|p| attribute(p).unwrap().1)
            .skip(1)
            .collect();
        Ok((&parts[0], Some(response)))
    }
}

fn get_parts(source: &str) -> IResult<&str, Option<Vec<&str>>> {
    let (remainder, mut parts) = many0(part)(source)?;
    parts.push(remainder);
    Ok(("", Some(parts)))
}

fn part(source: &str) -> IResult<&str, &str> {
    let (source, content) = take_until("|")(source)?;
    let (source, _) = tag("|")(source)?;
    Ok((source, content))
}

pub fn attribute(source: &str) -> IResult<&str, (Option<String>, Option<String>)> {
    let (v, k) = alt((tuple((take_until(":"), rest)), tuple((rest, rest))))(source)?;
    if k.1.is_empty() {
        Ok((v, (Some(k.0.trim().to_string()), None)))
    } else {
        let (v, _) = tag(":")(k.1)?;
        let (v, _) = multispace0(v)?;
        Ok((
            v,
            (Some(k.0.trim().to_string()), Some(v.trim().to_string())),
        ))
    }
}
