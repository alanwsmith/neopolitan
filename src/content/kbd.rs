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

pub fn kbd<'a>(source: (&'a str, &'a str, &'a str)) -> IResult<&'a str, Content> {
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
            Content::Kbd {
                attributes,
                text: Some(items[0].to_string()),
            },
        ))
    } else {
        Ok((
            "",
            Content::Kbd {
                attributes: None,
                text: Some(items[0].to_string()),
            },
        ))
    }
}
