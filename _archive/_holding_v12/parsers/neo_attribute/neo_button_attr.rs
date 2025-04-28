#![allow(unused_imports)]
use crate::parsers::neo_attribute::neo_attr_id::neo_attr_id;
use crate::parsers::neo_attribute::NeoAttribute;
use crate::parsers::neo_tag::css_class_name;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::alpha1;
use nom::character::complete::space1;
use nom::multi::separated_list1;
use nom::sequence::preceded;
use nom::sequence::terminated;
use nom::IResult;

pub fn neo_button_attr(
    source: &str,
) -> IResult<&str, NeoAttribute> {
    let (source, attr_key) = preceded(
        tag("|"),
        terminated(
            alt((tag("type"), tag("tktktktk"))),
            tag(": "),
        ),
    )(source)?;
    match attr_key {
        "type" => {
            let (source, value) = alt((
                tag("button"),
                tag("reset"),
                tag("submit"),
            ))(source)?;
            Ok((
                source,
                NeoAttribute::ButtonType(value.to_string()),
            ))
        }
        _ => panic!("Panic In Button Attr"),
    }
}

#[cfg(test)]
mod test {

    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("|type: reset>>", (">>", NeoAttribute::ButtonType("reset".to_string())))]
    #[case("|type: submit|class: alfa>>", ("|class: alfa>>", NeoAttribute::ButtonType("submit".to_string())))]
    fn neo_button_attr_test(
        #[case] input: &str,
        #[case] expected: (&str, NeoAttribute),
    ) {
        assert_eq!(
            expected,
            neo_button_attr(input).unwrap()
        );
    }

    //
}
