#![allow(unused_imports)]
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
use crate::tag_attrs::id::id;
use nom::multi::many0;
use crate::tags::TagAttr;
use nom::character::complete::space1;
use nom::multi::separated_list1;

pub fn code_class(source: &str) -> IResult<&str, Vec<String>> {
    let (source, value_string) =
        preceded(tag("|class: "), is_not("|>"))(source)?;
    let (_, classes) = separated_list1(
        space1,
        is_not(" ").map(|s: &str| s.to_string()),
    )(value_string)?;
    Ok((source, classes))
}

// pub fn code_tag_attrs(source: &str) -> IResult<&str, Vec<TagAttr>> {
//     let (source, attrs) = many0(alt((code_class, id)))(source)?;
//     Ok((source, attrs))
// }

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

    // dbg!(&source);

    // let mut the_classes: Vec<String> = vec![];

    // match lang {
    //     Some(x) => { the_classes.push(x.to_string()) },
    //     None => {}
    // }
    // dbg!(the_classes);


    // dbg!(&lang);

    let (source, mut attrs) = tag_attrs(source)?;
    let (source, _) = tag(">>")(source)?;

    attrs.iter_mut().for_each(|x| {
        match x {
            TagAttr::Class(x) => {
                match lang.clone() {
                    Some(l) => {
                        x.push(l)
                    }
                    None => {}
                }
            },
            _ => {}
        }
    });

     dbg!(&attrs);

     Ok((source, Tag::Code { attrs, text}))
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::tag_attrs::TagAttr;
    use rstest::rstest;

    #[rstest]
    #[ignore]
    #[case(
        "<<foxtrot|code>>",
        Tag::Code{ attrs: vec![], text: "foxtrot".to_string() }
    )]
    #[ignore]
    #[case(
        "<<sierra|code|rust>>",
        Tag::Code{ 
            attrs: vec![
                TagAttr::Class(vec!["language-rust".to_string()])
            ], 
            text: "sierra".to_string() }
    )]
    #[case(
        "<<foxtrot|code|python|id: bravo|class: echo>>",
        Tag::Code{ attrs: vec![
            TagAttr::Id("bravo".to_string()),
            TagAttr::Class(vec!["echo".to_string(), "python".to_string()])
        ],  text: "foxtrot".to_string() }
    )]
    #[ignore]
    #[case(
        "<<bravo|code|js|id: delta>>",
        Tag::Code{ attrs: vec![
            TagAttr::Id("delta".to_string())
        ],  text: "bravo".to_string() }
    )]
    #[ignore]
    #[case(
        "<<alfa|code|class: echo>>",
        Tag::Code{ 
            attrs: vec![
                TagAttr::Class(vec!["echo".to_string()])
            ], 
            text: "alfa".to_string() }
    )]
    fn solo_link_test(#[case] i: &str, #[case] e: Tag) {
        assert_eq!(e, code(i).unwrap().1);
    }

}
