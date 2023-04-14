use crate::attribute::*;
use crate::content::content::Content;
use nom::bytes::complete::is_not;
use nom::bytes::complete::tag;
use nom::multi::separated_list0;
use nom::IResult;

pub fn link<'a>(
    source: (&'a str, &'a str, &'a str, &'a str, &'a str),
) -> IResult<&'a str, Content> {
    let (_, items) = separated_list0(tag("|"), is_not("|"))(source.3)?;
    if items.len() > 1 {
        let attributes: Option<Vec<Attribute>> = Some(
            items
                .iter()
                .skip(1)
                .map(|a| attribute(a).unwrap().1)
                .collect(),
        );
        Ok((
            "",
            Content::Link {
                attributes,
                url: Some(items[0].to_string()),
                text: Some(source.1.to_string()),
            },
        ))
    } else {
        Ok((
            "",
            Content::Link {
                attributes: None,
                url: Some(items[0].to_string()),
                text: Some(source.1.to_string()),
            },
        ))
    }
}
