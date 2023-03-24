use nom::IResult;
use std::collections::HashMap;

// NOTE: This is the origianl one that returns
// a vec. It will go away when everyting is moved
// over to using a hashmap

pub fn text_attributes(_source: &str) -> IResult<&str, Option<HashMap<String, Option<String>>>> {
    Ok((
        "",
        Some(HashMap::from([(
            "id".to_string(),
            Some("rider".to_string()),
        )])),
    ))
    // let payload: Vec<(Option<String>, Option<String>)> = vec![];
    // let (remainder, mut parts) = many0(part)(source)?;
    // parts.push(remainder);
    // if parts[0].is_empty() {
    //     Ok(("", None))
    // } else {
    //     let response: Vec<(Option<String>, Option<String>)> =
    //         parts.iter().map(|p| attribute(p).unwrap().1).collect();
    //     Ok(("", Some(response)))
    // }
}

// pub fn text_attributes_(source: &str) -> IResult<&str, Option<Vec<(String, Option<String>)>>> {
//     let payload: Vec<(Option<String>, Option<String>)> = vec![];
//     let (remainder, mut parts) = many0(part)(source)?;
//     parts.push(remainder);
//     if parts[0].is_empty() {
//         Ok(("", None))
//     } else {
//         let response: Vec<(String, Option<String>)> =
//             parts.iter().map(|p| attribute(p).unwrap().1).collect();
//         Ok(("", Some(response)))
//     }
// }

// fn get_parts(source: &str) -> IResult<&str, Option<Vec<&str>>> {
//     let (remainder, mut parts) = many0(part)(source)?;
//     parts.push(remainder);
//     Ok(("", Some(parts)))
// }

// fn part(source: &str) -> IResult<&str, &str> {
//     let (source, content) = take_until("|")(source)?;
//     let (source, _) = tag("|")(source)?;
//     Ok((source, content))
// }

// pub fn attribute(source: &str) -> IResult<&str, (String, Option<String>)> {
//     let (v, k) = alt((tuple((take_until(":"), rest)), tuple((rest, rest))))(source)?;
//     if k.1.is_empty() {
//         Ok((v, (k.0.trim().to_string(), None)))
//     } else {
//         let (v, _) = tag(":")(k.1)?;
//         let (v, _) = multispace0(v)?;
//         Ok((v, (k.0.trim().to_string(), Some(v.trim().to_string()))))
//     }
// }
