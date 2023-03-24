#![allow(warnings)]
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::bytes::complete::take_until;
use nom::character::complete::multispace0;
use nom::combinator::rest;
use nom::multi::many0;
use nom::sequence::tuple;
use nom::IResult;
use std::collections::HashMap;

#[derive(Debug, PartialEq)]
pub struct Attributes {
    pub values: Option<Vec<(Option<String>, Option<String>)>>,
}

pub fn attributes(source: &str) -> IResult<&str, Option<HashMap<String, Option<String>>>> {
    let (remainder, source) = take_until("\n\n")(source)?;
    let (_, parts) = many0(part)(source)?;
    let mut response: HashMap<String, Option<String>> = HashMap::new();
    parts.iter().for_each(|p| {
        let (key, value) = attribute(p).unwrap();
        response.insert(key.to_string(), value);
    });

    // if final_part.is_empty() {
    //     Ok(("", None))
    // } else {
    //     // parts.push(final_part);
    //     // dbg!(&parts);
    //     let mut response: HashMap<String, Option<String>> = HashMap::new();
    //     response.insert("box".to_string(), Some("planks".to_string()));
    //     Ok(("", Some(response)))
    // }

    // response.insert("box".to_string(), Some("planks".to_string()));
    Ok(("", Some(response)))
}

pub fn attribute(source: &str) -> IResult<&str, Option<String>> {
    let (remainder, key) = take_until(": ")(source)?;
    let (value, _) = tag(": ")(remainder)?;
    Ok((key, Some(value.to_string())))
}

// pub fn attributes(source: &str) -> IResult<&str, Option<Vec<(Option<String>, Option<String>)>>> {
//     Ok(("", None))
// }

fn part(source: &str) -> IResult<&str, &str> {
    // dbg!(&source);
    // dbg!("------------");
    let (remainder, content) = tag(">> ")(source)?;
    // dbg!(&content);
    // dbg!(&remainder);
    let (remainder, content) = alt((take_until("\n"), rest))(remainder)?;
    // dbg!(&remainder);
    // dbg!(&content);

    // let (remainder, content) = take_until(">> ")(content)?;
    // let (source, _) = tag(">> ")(source)?;
    Ok((remainder, content))
}

// pub fn attribute_v2(source: &str) -> IResult<&str, (Option<String>, Option<String>)> {
//     let (v, k) = alt((tuple((take_until(":"), rest)), tuple((rest, rest))))(source)?;
//     if k.1.is_empty() {
//         Ok((v, (Some(k.0.trim().to_string()), None)))
//     } else {
//         let (v, _) = tag(":")(k.1)?;
//         let (v, _) = multispace0(v)?;
//         Ok((
//             v,
//             (Some(k.0.trim().to_string()), Some(v.trim().to_string())),
//         ))
//     }
// }

// pub fn attributes(source: &str) -> IResult<&str, Option<Vec<(Option<String>, Option<String>)>>> {
//     let (remainder, source) = take_until("\n\n")(source)?;
//     let (final_part, mut parts) = many0(part)(source)?;
//     if final_part.is_empty() {
//         parts.push(final_part);
//         let mut attribute_holder: Vec<(Option<String>, Option<String>)> = vec![];
//         for part in parts.iter().skip(1) {
//             let (_, b) = attribute(part)?;
//             attribute_holder.push(b);
//         }
//         Ok((remainder.trim(), None))
//     } else {
//         parts.push(final_part);
//         let mut attribute_holder: Vec<(Option<String>, Option<String>)> = vec![];
//         for part in parts.iter().skip(1) {
//             let (_, b) = attribute(part)?;
//             attribute_holder.push(b);
//         }
//         Ok((remainder.trim(), Some(attribute_holder)))
//     }
// }
