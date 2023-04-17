use crate::attribute::*;
use crate::content::content::Content;
use nom::branch::alt;
use nom::bytes::complete::is_not;
use nom::bytes::complete::tag;
use nom::bytes::complete::take_until;
use nom::character::complete::multispace0;
use nom::combinator::rest;
use nom::multi::separated_list0;
use nom::sequence::tuple;
use nom::IResult;
use nom::Parser;

pub fn neo_tag_attributes(source: &str) -> IResult<&str, Option<Vec<Attribute>>> {
    let (_, parts) = separated_list0(tag("|"), is_not("|"))(source)?;
    dbg!(&parts);
    if parts.len() > 1 {
        let attributes: Option<Vec<Attribute>> = Some(
            parts
                .iter()
                .skip(1)
                .map(|a| attribute(a).unwrap().1)
                .collect(),
        );
        Ok(("", attributes))
    } else {
        Ok(("", None))
    }
}

pub fn neo_tag<'a>(source: (&'a str, &'a str, &'a str)) -> IResult<&'a str, Content> {
    let (_, parts) = tuple((take_until("|"), tag("|"), rest))(source.1)?;
    let (a, b) = alt((
        tuple((multispace0, tag("strong"), multispace0)).map(|_| Content::Strong {
            attributes: neo_tag_attributes(parts.2).unwrap().1,
            text: Some(parts.0.to_string()),
        }),
        tuple((multispace0, tag("b"), multispace0)).map(|_| Content::B {
            attributes: neo_tag_attributes(parts.2).unwrap().1,
            text: Some(parts.0.to_string()),
        }),
    ))(parts.2)?;
    Ok((a, b))
}

#[cfg(test)]
mod test {

    use crate::attribute::*;
    use crate::content::content::*;
    use crate::content::neo_tag::neo_tag;

    #[test]
    fn b() {
        // let lines = vec!["alfa <<bravo|strong|class: delta>> charlie"].join("\n");
        let source = ("", "alfa|b", "");
        let expected = Content::B {
            attributes: None,
            text: Some("alfa".to_string()),
        };
        let result = neo_tag(source);
        assert_eq!(expected, result.unwrap().1);
    }

    #[test]
    fn b_with_one_attribute() {
        // let lines = vec!["alfa <<bravo|strong|class: delta>> charlie"].join("\n");
        let source = ("", "bravo|b|class: charlie", "");
        let expected = Content::B {
            attributes: Some(vec![Attribute::Basic {
                key: Some("class".to_string()),
                value: Some("charlie".to_string()),
            }]),
            text: Some("bravo".to_string()),
        };
        let result = neo_tag(source);
        assert_eq!(expected, result.unwrap().1);
    }

    // "<<b|delta>> <<b|echo>> <<b|foxtrot>>",
    // "<<b|golf|class: advanced>>",
    // "<<code|quick brown>>",
    // "<<code|slow fox|lang: rust>>",
    // "<<code|slow fox|rust>>",
    // "<<em|some flower seeds>>",
    // "<<i|red ink>>",
    // "<<kbd|ctrl>>",
    // "<<span|the worn floor>>",
    // "<<strike|the worn floor>>",
    // "<<strong|the worn floor>>",
    // "<<sub|the worn floor>>",
    // "<<sup|the worn floor>>",
    // "<<u|the worn floor>>",

    #[test]
    fn strong() {
        // let lines = vec!["alfa <<bravo|strong|class: delta>> charlie"].join("\n");
        let source = ("", "bravo|strong|class: delta", "");
        let expected = Content::Strong {
            attributes: Some(vec![Attribute::Basic {
                key: Some("class".to_string()),
                value: Some("delta".to_string()),
            }]),
            text: Some("bravo".to_string()),
        };
        let result = neo_tag(source);
        assert_eq!(expected, result.unwrap().1);
    }
}
