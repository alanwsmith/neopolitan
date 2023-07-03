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
use nom::bytes::complete::is_not;

pub fn neo_data_attr(
    source: &str,
) -> IResult<&str, NeoAttribute> {
    let (source, value) = preceded(
        tag("|value: "),
            is_not(">|")
    )(source)?;

    Ok((
        source,
        NeoAttribute::DataValue(value.to_string()),
    ))
}

#[cfg(test)]
mod test {

    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("|value: alfa bravo>>", (">>", NeoAttribute::DataValue("alfa bravo".to_string())))]
    fn solo_neo_data_attr_test(
        #[case] input: &str,
        #[case] expected: (&str, NeoAttribute),
    ) {
        assert_eq!(
            expected,
            neo_data_attr(input).unwrap()
        );
    }

    //
}
