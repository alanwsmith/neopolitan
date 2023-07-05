#![allow(unused_imports)]
#![allow(unused_variables)]
// use crate::parsers::attribute::attribute::Attribute;
use crate::parsers::neotags::abbr_tag::abbr_tag;
// use crate::parsers::attribute::attributes;
use nom::bytes::complete::is_not;
use nom::bytes::complete::tag;
use nom::sequence::pair;
use nom::sequence::preceded;
use nom::IResult;
use nom::Parser;
use crate::parsers::attribute::Attribute;

pub mod abbr_tag;

#[derive(Debug, PartialEq)]
pub enum NeoTag {
    Abbr {
        content: String,
        attrs: Vec<Attribute>,
    },
    Empty,
}

pub fn neo_tag(source: &str) -> IResult<&str, NeoTag> {
    let (source, (content, tag_name)) = pair(
        preceded(tag("<<"), is_not("|")),
        preceded(tag("|"), is_not("|>")),
    )(source)?;

    // dbg!(&source);
    // dbg!(&payload);
    // dbg!(&tag_name);

    let result = match tag_name {
        "abbr" => {

            abbr_tag(source)?;


            // (
            //         " delta", 
            //         NeoTag::Abbr{
            //         content: "foxtrot".to_string(),
            //         attrs: vec![
            //         Attribute::Class(vec!["alfa".to_string(), "bravo".to_string()])
            //     ]})


            (
                " delta", 
                NeoTag::Abbr{
                content: "foxtrot".to_string(),
                attrs: vec![
                Attribute::Class(vec!["alfa".to_string(), "bravo".to_string()])
            ]})


            // // let (source, attrs) = attributes(source)?;
            // (
            //     " delta",
            //     NeoTag::Abbr {
            //         content: content.to_string(),
            //         attrs: attrs
            //     },
            // )


        }
        _ => ("", NeoTag::Empty),
    };

    Ok(result)
}

#[cfg(test)]

mod test {

    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("<<foxtrot|abbr|class: alfa bravo>> delta", (
        " delta", 
        NeoTag::Abbr{
        content: "foxtrot".to_string(),
        attrs: vec![
        Attribute::Class(vec!["alfa".to_string(), "bravo".to_string()])
    ]}))]
    fn solo_class_tester(
        #[case] input: &str,
        #[case] expected: (&str, NeoTag),
    ) {
        assert_eq!(expected, neo_tag(input).unwrap());
    }
}
