use nom::bytes::complete::is_not;
use nom::bytes::complete::tag;
use nom::combinator::opt;
use nom::multi::separated_list0;
use nom::IResult;
use std::collections::HashMap;

pub fn parse_text_attributes(
    source: &str,
) -> IResult<&str, Option<HashMap<String, Option<String>>>> {
    let (_, parts) = split_parts(source, "|")?;
    if parts.is_empty() {
        Ok(("", None))
    } else {
        let mut response: HashMap<String, Option<String>> = HashMap::new();
        parts.iter().for_each(|p| {
            let (_, tokens) = split_parts(p, ":").unwrap();
            if tokens.len() == 1 {
                response.insert(tokens[0].trim().to_string(), None);
            } else {
                response.insert(
                    tokens[0].trim().to_string(),
                    Some(tokens[1].trim().to_string()),
                );
            }
        });
        Ok(("", Some(response)))
    }
}

fn split_parts<'a>(source: &'a str, separator: &'a str) -> IResult<&'a str, Vec<&'a str>> {
    let (remainder, content) = opt(tag(separator))(source)?;
    match content {
        None => {
            let (_, items) = separated_list0(tag(separator), is_not(separator))(remainder)?;
            Ok(("", items))
        }
        Some(..) => {
            let (_, items) = separated_list0(tag(separator), is_not(separator))(remainder)?;
            Ok(("", items))
        }
    }
}
