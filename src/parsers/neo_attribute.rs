#![allow(unused_imports)]
use crate::parsers::neo_attribute::neo_attr_id::neo_attr_id;
use crate::parsers::neo_tag::css_class_name;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::alpha1;
use nom::character::complete::space1;
use nom::multi::separated_list1;
use nom::sequence::preceded;
use nom::sequence::terminated;
use nom::IResult;

pub mod neo_attr_id;

#[derive(Debug, PartialEq)]
pub enum NeoAttribute {
    Class(Vec<String>),
    Id(String),
}

pub fn neo_attribute(
    source: &str,
) -> IResult<&str, NeoAttribute> {
    let (source, attr_key) = preceded(
        tag("|"),
        terminated(
            alt((tag("class"), tag("id"))),
            tag(": "),
        ),
    )(source)?;
    match attr_key {
        "class" => {
            let (source, attr_values) =
                separated_list1(space1, css_class_name)(
                    source,
                )?;
            Ok((source, NeoAttribute::Class(attr_values)))
        }
        "id" => {
            let (source, attr_value) = neo_attr_id(source)?;
            Ok((
                source,
                NeoAttribute::Id(attr_value.to_string()),
            ))
        }
        _ => panic!("AAAAAAAAAAAAAAAAAAAAAAAAAA"),
    }
}

#[cfg(test)]
mod test {

    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("|class: alfa>>", (">>", NeoAttribute::Class(vec!["alfa".to_string()])))]
    #[case("|class: bravo charlie>>", (">>", NeoAttribute::Class(vec!["bravo".to_string(), "charlie".to_string()])))]
    #[case("|id: delta>>", (">>", NeoAttribute::Id("delta".to_string())))]
    #[case("|id: echo|class: foxtrot golf>>", ("|class: foxtrot golf>>", NeoAttribute::Id("echo".to_string())))]
    fn solo_neo_attribute_test(
        #[case] input: &str,
        #[case] expected: (&str, NeoAttribute),
    ) {
        assert_eq!(expected, neo_attribute(input).unwrap());
    }

    #[test]
    #[ignore]
    pub fn tktktktkt() {
        // TODO: Make these work:
        // #[case("class: _weird1-name", ("", NeoAttribute::Class("_weird1-name".to_string())])))]
        // #[case("contenteditable: true", ("", NeoAttribute::ContentEditable(true)])))]
        // #[case("data-foxtrot: alfa bravo", ("", NeoAttribute::Data(TBD: key, value probably)))]
        // #[case("hidden", ("", NeoAttribute::Hidden))]
        // #[case("spellcheck: true", ("", NeoAttribute::SpellCheck(true)])))]
        // #[case("style: margin: 10px; line-height: 20px;", ("", NeoAttribute::Style(TBD: )))]
        // That's the items from the list of global attributes from MDN. Indiviudal
        // elements have other things as well.
    }

    //
}
