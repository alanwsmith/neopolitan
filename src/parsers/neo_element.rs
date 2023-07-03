#![allow(unused_imports)]
use crate::parsers::global_attr::GlobalAttr;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::bytes::complete::tag_no_case;
use nom::character::complete::alpha1;
use nom::sequence::separated_pair;
use nom::IResult;
use nom::Parser;

#[derive(Debug, PartialEq)]
pub enum NeoElement {
    Keyboard,
    Link { url: String },
    Strong { global_attrs: Vec<GlobalAttr> },
}

pub fn neo_element(
    source: &str,
) -> IResult<&str, NeoElement> {
    let (source, el) = alt((
        tag_no_case("strong").map(|_| NeoElement::Strong {
            global_attrs: vec![],
        }),
        // tag_no_case("kbd").map(|_| NeoElement::Keyboard),
        // separated_pair(
        //     tag_no_case("link"),
        //     tag("|"),
        //     alpha1,
        // )
        // .map(|(_, url): (&str, &str)| {
        //     NeoElement::Link {
        //         url: url.to_string(),
        //     }
        // }),
    ))(source)?;
    Ok((source, el))
}

#[cfg(test)]
mod test {

    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("strong", ("", NeoElement::Strong { global_attrs: vec![]}))]
    // #[case("kbd|class: sierra", ("|class: sierra", NeoElement::Keyboard))]
    // #[case("link|localhost", ("", NeoElement::Link{url: "localhost".to_string()}))]
    //#[case("code>>", ("", NeoElement::Code{language: None }))]
    // #[case("code|rust>>", ("", NeoElement::Code{language: Some("rust".to_string())}))]
    // #[case("code|class: alfa>>", ("", NeoElement::Code{language: None}))]
    //#[case("b>>", ("", NeoElement::Code{language: None }))]
    fn solo_neo_element_test(
        #[case] input: &str,
        #[case] expected: (&str, NeoElement),
    ) {
        assert_eq!(expected, neo_element(input).unwrap());
    }

    //
}
