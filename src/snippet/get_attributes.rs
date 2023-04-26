use nom::bytes::complete::is_not;
use nom::bytes::complete::tag;
use nom::combinator::opt;
use nom::multi::separated_list0;
use nom::IResult;

pub fn get_attributes(source: &str) -> IResult<&str, Option<String>> {
    let mut response_string = String::from("");
    let (_, parts) = split(source, "|")?;
    parts.iter().for_each(|x| {
        let (_, b) = split(x, ":").unwrap();
        if b.len() > 1 {
            response_string.push_str(format!(r#" {}="{}""#, b[0].trim(), b[1].trim()).as_str());
        } else {
            response_string.push_str(format!(r#" {}"#, b[0].trim()).as_str());
        }
    });
    Ok(("", Some(response_string)))
}

fn split<'a>(source: &'a str, separator: &'a str) -> IResult<&'a str, Vec<&'a str>> {
    let (remainder, _) = opt(tag(separator))(source)?;
    let (_, items) = separated_list0(tag(separator), is_not(separator))(remainder)?;
    Ok(("", items))
}
