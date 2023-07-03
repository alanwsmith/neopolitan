use crate::parsers::neo_tag::css_class_name;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::alpha1;
use nom::character::complete::space1;
use nom::multi::separated_list1;
use nom::sequence::terminated;
use nom::IResult;

#[derive(Debug, PartialEq)]
pub enum NeoAttribute {
    Class(Vec<String>),
    Id(String),
}

pub fn neo_attribute(
    source: &str,
) -> IResult<&str, NeoAttribute> {
    let (source, attr_key) = terminated(
        alt((tag("class"), tag("id"))),
        tag(": "),
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
            let (source, attr_value) = alpha1(source)?;
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
    #[case("class: alfa", ("", NeoAttribute::Class(vec!["alfa".to_string()])))]
    fn solo_neo_attribute_test(
        #[case] input: &str,
        #[case] expected: (&str, NeoAttribute),
    ) {
        assert_eq!(expected, neo_attribute(input).unwrap());
    }
}
