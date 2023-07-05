use crate::tags::Tag;
use nom::bytes::complete::is_not;
use nom::bytes::complete::tag;
use nom::bytes::complete::tag_no_case;
use nom::sequence::tuple;
use nom::sequence::delimited;
use nom::IResult;
use crate::tag_attrs::tag_attrs;
use nom::Parser;

pub fn basic<'a>(source: &'a str, name: &'a str) -> IResult<&'a str, Tag> {
    let (source, text) =
        delimited(tag("<<"), is_not("|").map(|s: &str| s.to_string()), tuple((tag("|"), tag_no_case(name))))(source)?;
    let (source, attrs) = tag_attrs(source)?;
    let (source, _) = tag(">>")(source)?;
    match name {
        "abbr" => {
            Ok((source, Tag::Abbr { text, attrs} ))
        },
        "strong" => {
            Ok((source, Tag::Strong { text, attrs} ))
        },
        _ => panic!("No tag")
    } 
}

#[cfg(test)]

mod test{
    use super::*;
    use rstest::rstest;
    use nom::error::Error;
    use nom::Err;
    use crate::tags::TagAttr;

    #[rstest]
    #[case(
        "<<delta|abbr>>", 
        "abbr",
        Ok(("", Tag::Abbr{ attrs: vec![], text: "delta".to_string() })))]
    #[case(
        "<<alfa bravo|strong>>", 
        "strong",
        Ok(("", Tag::Strong{ attrs: vec![], text: "alfa bravo".to_string() })))]
    #[case(
        "<<alfa bravo|strong|class: charlie delta>>", 
        "strong",
        Ok(("", 
        Tag::Strong{ 
                attrs: vec![
                    TagAttr::Class(
                        vec![
                            "charlie".to_string(), "delta".to_string()
                        ]
                    )
                ],
                text: "alfa bravo".to_string() 
            })))]
    fn strong_runner(#[case] i1: &str, #[case] i2: &str, #[case] e: Result<(&str, Tag), Err<Error<&str>>>) {
        assert_eq!(e, basic(i1, i2))
    }
}
