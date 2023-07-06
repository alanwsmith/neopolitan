use crate::tag_attrs::tag_attrs;
use crate::tags::Tag;
use nom::bytes::complete::is_not;
use nom::bytes::complete::tag;
use nom::bytes::complete::tag_no_case;
use nom::combinator::opt;
use nom::sequence::delimited;
use nom::sequence::preceded;
use nom::sequence::tuple;
use nom::sequence::terminated;
use nom::branch::alt;
use nom::IResult;
use nom::Parser;
use nom::combinator::peek;

pub fn code(source: &str) -> IResult<&str, Tag> {
    let (source, text) = delimited(
        tag("<<"),
        is_not("|").map(|s: &str| s.to_string()),
        tuple((tag("|"), tag_no_case("code"))),
    )(source)?;
    let (source, lang) = opt(
        preceded(
            tag("|"),
            terminated(
                is_not("|:>").map(|s: &str| s.to_string()),
                alt((
                    peek(tag("|")), peek(tag(">"))
                ))
            )
        )
    )(source)?;
    dbg!(&lang);
    let (source, attrs) = tag_attrs(source)?;
    let (source, _) = tag(">>")(source)?;

    Ok((source, Tag::Code { attrs, lang, text }))
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::tag_attrs::TagAttr;
    use rstest::rstest;

    #[rstest]
    #[case(
        "<<foxtrot|code>>",
        Tag::Code{ attrs: vec![], lang: None, text: "foxtrot".to_string() }
    )]
    #[case(
        "<<sierra|code|rust>>",
        Tag::Code{ attrs: vec![], lang: Some("rust".to_string()), text: "sierra".to_string() }
    )]
    #[case(
        "<<bravo|code|js|id: delta>>",
        Tag::Code{ attrs: vec![
            TagAttr::Id("delta".to_string())
        ], lang: Some("js".to_string()), text: "bravo".to_string() }
    )]
    #[case(
        "<<alfa|code|class: echo>>",
        Tag::Code{ 
            attrs: vec![
                TagAttr::Class(vec!["echo".to_string()])
            ], 
            lang: None, 
            text: "alfa".to_string() }
    )]
    fn solo_link_test(#[case] i: &str, #[case] e: Tag) {
        assert_eq!(e, code(i).unwrap().1);
    }
}
