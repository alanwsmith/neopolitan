use crate::attribute::*;
use crate::content::content::Content;
use nom::bytes::complete::is_not;
use nom::bytes::complete::tag;
use nom::multi::separated_list0;
use nom::IResult;

pub fn span<'a>(source: (&'a str, &'a str, &'a str)) -> IResult<&'a str, Content> {
    let (_, items) = separated_list0(tag("|"), is_not("|"))(source.1)?;
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
            Content::Span {
                attributes,
                text: Some(items[0].to_string()),
            },
        ))
    } else {
        Ok((
            "",
            Content::Span {
                attributes: None,
                text: Some(items[0].to_string()),
            },
        ))
    }
}
