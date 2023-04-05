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

pub fn code<'a>(source: (&'a str, &'a str, &'a str)) -> IResult<&'a str, Content> {
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
            Content::Code {
                attributes,
                text: Some(items[0].to_string()),
            },
        ))
    } else {
        Ok((
            "",
            Content::Code {
                attributes: None,
                text: Some(items[0].to_string()),
            },
        ))
    }
}

// #[cfg(test)]
// mod test {
//     #[test]
//     fn basic() {
//         let lines = vec!["<<b|text>>"].join("\n");
//         let source = lines.as_str();
//         let expected = Wrapper::Page {
//             children: Some(vec![Section::Title {
//                 attributes: Some(vec![SectionAttribute::Attribute {
//                     key: Some("id".to_string()),
//                     value: Some("bravo".to_string()),
//                 }]),
//                 children: None,
//             }]),
//         };
//         let result = parse(source).unwrap().1;
//         assert_eq!(expected, result);
//     }
// }
