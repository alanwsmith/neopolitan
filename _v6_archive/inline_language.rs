use crate::split::split;
use nom::IResult;

pub fn inline_language(source: &str) -> IResult<&str, Option<String>> {
    let (_, parts) = split(source, "|")?;
    if parts.len() == 0 {
        Ok(("", None))
    } else {
        Ok(("", Some(parts[0].trim().to_string())))
    }
}
