#![allow(unused_imports)]
use nom::bytes::complete::tag;
use nom::character::complete::{alpha1, digit1};
use nom::multi::separated_list1;
use nom::sequence::tuple;
use nom::IResult;
use nom::Parser;

#[derive(Debug, PartialEq)]
pub struct Neotag {
    tag_name: String,
    content: String,
}

pub fn neotag(source: &str) -> IResult<&str, Neotag> {
    let (_, b) = tuple((
        tag("<<"),
        separated_list1(tag("|"), alpha1),
        tag(">>"),
    ))(source)?;
    // dbg!(&a);
    dbg!(&b);

    let nt = Neotag {
        tag_name: b.1[1].to_string(),
        content: b.1[0].to_string(),
    };

    //  let mut parser = tuple((alpha1, digit1, alpha1));
    // assert_eq!(
    //     parser("abc123def"),
    //     Ok(("", ("abc", "123", "def")))
    // );
    // let (remainder, value) =
    //     tuple((tag("<<"), tag(">>"), tag("::")))(source)?;
    Ok(("", nt))
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
