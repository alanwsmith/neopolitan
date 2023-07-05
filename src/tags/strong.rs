use crate::tags::Tag;
use nom::bytes::complete::is_not;
use nom::bytes::complete::tag;
use nom::bytes::complete::tag_no_case;
use nom::sequence::delimited;
use nom::IResult;
use crate::tag_attrs::tag_attrs;
use nom::Parser;

pub fn strong(source: &str) -> IResult<&str, Tag> {
    let (source, text) =
        delimited(tag("<<"), is_not("|").map(|s: &str| s.to_string()), tag_no_case("|strong"))(source)?;
    let (source, attrs) = tag_attrs(source)?;
    let (source, _) = tag(">>")(source)?;
    Ok((
        source,
        Tag::Strong { text, attrs},
    ))
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
        "<<alfa bravo|strong>>", 
        Ok(("", Tag::Strong{ attrs: vec![], text: "alfa bravo".to_string() })))]
    #[case(
        "<<alfa bravo|strong|class: charlie delta>>", 
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
    fn strong_runner(#[case] input: &str, #[case] expected: Result<(&str, Tag), Err<Error<&str>>>) {
        assert_eq!(expected, strong(input))
    }
}
