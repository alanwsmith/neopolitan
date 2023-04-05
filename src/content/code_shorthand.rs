use crate::attribute::*;
use crate::content::content::Content;
use nom::bytes::complete::is_not;
use nom::bytes::complete::tag;
use nom::multi::separated_list0;
use nom::IResult;

pub fn code_shorthand<'a>(
    source: (&'a str, &'a str, &'a str, &'a str, &'a str),
) -> IResult<&'a str, Content> {
    let (_, items) = separated_list0(tag("|"), is_not("|"))(source.3)?;
    if items.len() > 0 {
        let attributes: Option<Vec<Attribute>> =
            Some(items.iter().map(|a| attribute(a).unwrap().1).collect());
        Ok((
            "",
            Content::CodeShorthand {
                attributes,
                text: Some(source.1.to_string()),
            },
        ))
    } else {
        Ok((
            "",
            Content::CodeShorthand {
                attributes: None,
                text: Some(source.1.to_string()),
            },
        ))
    }
}
