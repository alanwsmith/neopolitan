#![allow(unused_imports)]
use crate::attribute::*;
use crate::content::content::Content;
use nom::branch::alt;
use nom::bytes::complete::is_not;
use nom::bytes::complete::tag;
use nom::bytes::complete::tag_no_case;
use nom::bytes::complete::take_till;
use nom::bytes::complete::take_until;
use nom::character::complete::multispace1;
use nom::error::Error;
use nom::multi::separated_list0;
use nom::sequence::tuple;
use nom::IResult;
use nom::Parser;

pub fn link<'a>(
    source: (&'a str, &'a str, &'a str, &'a str, &'a str),
) -> IResult<&'a str, Content> {
    Ok((
        "",
        Content::Link {
            attributes: None,
            url: Some(source.1.to_string()),
            text: Some(source.3.to_string()),
        },
    ))
}

pub fn link_dev<'a>(
    source: (&'a str, &'a str, &'a str, &'a str, &'a str),
) -> IResult<&'a str, Content> {
    // let (a, b) =
    //many0(tuple((take_until(token), tag(token), not_line_ending)).map(|x| x.2))(source)?;
    let (_, items) = separated_list0(tag("|"), is_not("|"))(source.3)?;
    dbg!(&items);
    if items.len() > 1 {
        let attributes: Option<Vec<Attribute>> = Some(
            items
                .iter()
                .skip(1)
                .map(|a| attribute(a).unwrap().1)
                .collect(),
        );
        dbg!(&attributes);
        Ok((
            "",
            Content::Link {
                attributes,
                url: Some(source.1.to_string()),
                text: Some(items[0].to_string()),
            },
        ))
    } else {
        Ok((
            "",
            Content::Link {
                attributes: None,
                url: Some(source.1.to_string()),
                text: Some(items[0].to_string()),
            },
        ))
    }
}

pub fn attribute(source: &str) -> IResult<&str, Attribute> {
    Ok((
        "",
        Attribute::Basic {
            key: Some("class".to_string()),
            value: Some("important".to_string()),
        },
    ))
}
