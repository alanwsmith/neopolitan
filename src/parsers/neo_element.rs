#![allow(unused_imports)]
use crate::parsers::neo_attribute::neo_attribute;
use crate::parsers::neo_attribute::neo_button_attr::neo_button_attr;
use crate::parsers::neo_attribute::NeoAttribute;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::bytes::complete::tag_no_case;
use nom::character::complete::alpha1;
use nom::multi::many0;
use nom::sequence::separated_pair;
use nom::IResult;
use nom::Parser;

#[derive(Debug, PartialEq)]
pub enum NeoElement {
    Abbreviation(Vec<NeoAttribute>),
    BringAttention(Vec<NeoAttribute>),
    Button(Vec<NeoAttribute>),
    Link(Vec<NeoAttribute>),
    Strong(Vec<NeoAttribute>),
    None,
}

pub fn neo_element(
    source: &str,
) -> IResult<&str, NeoElement> {
    let (source, el) = alt((
        tag_no_case("abbr"),
        tag_no_case("button"),
        tag_no_case("b"),
        tag_no_case("strong"),
    ))(source)?;

    match el {
        "abbr" => {
            let (source, attrs) =
                many0(neo_attribute)(source)?;
            Ok((source, NeoElement::Abbreviation(attrs)))
        }
        "b" => {
            let (source, attrs) =
                many0(neo_attribute)(source)?;
            Ok((
                source,
                NeoElement::BringAttention(attrs),
            ))
        }
        "button" => {
            let (source, attrs) = many0(alt((
                neo_attribute,
                neo_button_attr,
            )))(source)?;
            Ok((source, NeoElement::Button(attrs)))
        }
        "strong" => {
            let (source, attrs) =
                many0(neo_attribute)(source)?;
            Ok((source, NeoElement::Strong(attrs)))
        }
        _ => Ok((source, NeoElement::None)),
    }
}

pub fn global_attrs(
    source: &str,
) -> IResult<&str, Vec<NeoAttribute>> {
    Ok((source, vec![]))
}

#[cfg(test)]
mod test {

    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("abbr|id: delta>>", (">>", 
        NeoElement::Abbreviation(
            vec![NeoAttribute::Id("delta".to_string())])))]
    #[case("abbr|id: echo foxtrot>>", (">>", 
        NeoElement::Abbreviation(
            vec![NeoAttribute::Id("echo foxtrot".to_string())])))]
    #[case("abbr|class: sierra whiskey|id: echo foxtrot>>", (">>", 
        NeoElement::Abbreviation(vec![
            NeoAttribute::Class(vec!["sierra".to_string(), "whiskey".to_string()]), 
            NeoAttribute::Id("echo foxtrot".to_string()
        )])))]
    #[case("b>>", (">>", 
        NeoElement::BringAttention(vec![])))]
    #[case("button>>", (">>", 
        NeoElement::Button(vec![])))]
    #[case("button|type: reset>>", (">>", 
        NeoElement::Button(vec![NeoAttribute::ButtonType("reset".to_string())])))]
    #[case("strong>>", (">>", 
        NeoElement::Strong(vec![])))]
    #[case("strong|class: alfa>>", (">>", 
        NeoElement::Strong ( vec![
            NeoAttribute::Class(vec!["alfa".to_string()])])))]
    #[case("strong|class: bravo charlie>>", (">>", 
        NeoElement::Strong ( vec![
            NeoAttribute::Class(vec!["bravo".to_string(), "charlie".to_string()])])))]

    // #[case("strong", ("", NeoElement::Strong { global_attrs: vec![]}))]
    // #[case("link|localhost", ("", NeoElement::Link{url: "localhost".to_string()}))]
    //#[case("code>>", ("", NeoElement::Code{language: None }))]
    // #[case("code|rust>>", ("", NeoElement::Code{language: Some("rust".to_string())}))]
    // #[case("code|class: alfa>>", ("", NeoElement::Code{language: None}))]
    //#[case("b>>", ("", NeoElement::Code{language: None }))]
    fn neo_element_test(
        #[case] input: &str,
        #[case] expected: (&str, NeoElement),
    ) {
        assert_eq!(expected, neo_element(input).unwrap());
    }

    //
}
