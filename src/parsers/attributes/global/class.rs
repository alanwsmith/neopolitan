use crate::parsers::attributes::Attribute;
use nom::IResult;

pub fn class_attr(source: &str) -> IResult<&str, Attribute> {
    Ok((source, Attribute::Empty))
}

#[cfg(test)]
mod test {

    use super::*;
    use rstest::rstest;

    #[rstest]
    #[ignore]
    #[case("|class: alfa>>", (">>", Attribute::Class(vec!["alfa".to_string()])))]
    fn solo_class_tester(
        #[case] input: &str,
        #[case] expected: (&str, Attribute),
    ) {
        assert_eq!(expected, class_attr(input).unwrap());
    }

}