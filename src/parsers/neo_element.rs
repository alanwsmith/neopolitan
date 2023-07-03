#![allow(unused_imports)]
use crate::parsers::neo_attribute::neo_attribute;
use crate::parsers::neo_attribute::neo_button_attr::neo_button_attr;
use crate::parsers::neo_attribute::neo_data_attr::neo_data_attr;
use crate::parsers::neo_attribute::NeoAttribute;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::bytes::complete::tag_no_case;
use nom::character::complete::alpha1;
use nom::combinator::map_parser;
use nom::combinator::map_res;
use nom::multi::many0;
use nom::sequence::separated_pair;
use nom::IResult;
use nom::Parser;

#[derive(Debug, PartialEq)]
pub enum NeoElement {
    Abbr(Vec<NeoAttribute>),
    B(Vec<NeoAttribute>),
    Button(Vec<NeoAttribute>),
    Data(Vec<NeoAttribute>),
    Del(Vec<NeoAttribute>),
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
        tag_no_case("data"),
        tag_no_case("del"),
    ))(source)?;

    let payload = match el {
        "abbr" => {
            let (source, attrs) =
                many0(neo_attribute)(source)?;
            (source, NeoElement::Abbr(attrs))
        }
        "b" => {
            let (source, attrs) =
                many0(neo_attribute)(source)?;
            (source, NeoElement::B(attrs))
        }
        "button" => {
            let (source, attrs) = many0(alt((
                neo_attribute,
                neo_button_attr,
            )))(source)?;
            (source, NeoElement::Button(attrs))
        }
        "data" => {
            let (source, attrs) = many0(alt((
                neo_attribute,
                neo_data_attr,
            )))(source)?;
            (source, NeoElement::Data(attrs))
        }
        "del" => {
            let (source, attrs) =
                many0(neo_attribute)(source)?;
            (source, NeoElement::Del(attrs))
        }
        "strong" => {
            let (source, attrs) =
                many0(neo_attribute)(source)?;
            (source, NeoElement::Strong(attrs))
        }
        _ => (source, NeoElement::None),
    };
    Ok(payload)
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
        NeoElement::Abbr(
            vec![NeoAttribute::Id("delta".to_string())])))]
    #[case("abbr|id: echo foxtrot>>", (">>",
        NeoElement::Abbr(
            vec![NeoAttribute::Id("echo foxtrot".to_string())])))]
    #[case("abbr|class: sierra whiskey|id: echo foxtrot>>", (">>",
        NeoElement::Abbr(vec![
            NeoAttribute::Class(vec!["sierra".to_string(), "whiskey".to_string()]),
            NeoAttribute::Id("echo foxtrot".to_string()
        )])))]
    #[case("b>>", (">>",
        NeoElement::B(vec![])))]
    #[case("button>>", (">>",
        NeoElement::Button(vec![])))]
    #[case("button|type: reset>>", (">>",
        NeoElement::Button(vec![NeoAttribute::ButtonType("reset".to_string())])))]
    #[case("data>>", (">>",
        NeoElement::Data(vec![])))]
    #[case("data|value: foxtrot>>", (">>",
        NeoElement::Data(vec![
            NeoAttribute::DataValue("foxtrot".to_string())
        ])))]
    #[case("del>>", (">>",
        NeoElement::Del(vec![])))]
    // - dfn - The Definition Element
    // - em - The Emphasis Element
    // - i - The Idiomatic Text Element
    // - ins - The Insert Element
    // - kbd - The Keyboard Input Element
    // - label - The Label Element
    // - legend - The Field Set Legend Element
    // - link - The External Resource Link Element
    // - meter - The HTML Meter Element
    // - object - The External Object Element
    // - progress - The Progress Indicator Element
    // - q - The Inline Quotation Element
    // - s - The Strikethrough Element
    // - samp - The Sample Output Element
    // - small - The Side Comment Element
    // - span - The Content Span Element
    // - strong - The Strong Importance Element

    ////
    //#[case("strong>>", (">>",
    //    NeoElement::Strong(vec![])))]
    //#[case("strong|class: alfa>>", (">>",
    //    NeoElement::Strong ( vec![
    //        NeoAttribute::Class(vec!["alfa".to_string()])])))]
    //#[case("strong|class: bravo charlie>>", (">>",
    //    NeoElement::Strong ( vec![
    //        NeoAttribute::Class(vec!["bravo".to_string(), "charlie".to_string()])])))]

    // - sub - The Subscript Element
    // - sup - The Superscript Element
    // - time - The (Date) Time Element
    // - u - The Unarticulated Annotation Element
    // - var - The Variable element

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
