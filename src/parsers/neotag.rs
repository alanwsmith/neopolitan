#![allow(unused_imports)]
use nom::bytes::complete::tag;
use nom::character::complete::{alpha1, digit1};
use nom::multi::separated_list1;
use nom::sequence::delimited;
use nom::sequence::tuple;
use nom::IResult;
use nom::Parser;

#[derive(Debug, PartialEq)]
pub struct Neotag {
    tag_name: String,
    content: String,
}

pub fn neotag(source: &str) -> IResult<&str, Neotag> {
    let (source, b) = delimited(
        tag("<<"),
        separated_list1(tag("|"), alpha1),
        tag(">>"),
    )(source)?;
    let nt = Neotag {
        tag_name: b[1].to_string(),
        content: b[0].to_string(),
    };
    Ok((source, nt))
}

#[cfg(test)]

mod test {

    use super::*;

    #[test]
    pub fn solo_basic_neotag() {
        assert_eq!(
            neotag("<<alfa|strong>>").unwrap(),
            (
                "",
                Neotag {
                    tag_name: "strong".to_string(),
                    content: "alfa".to_string()
                }
            )
        );
    }
}
