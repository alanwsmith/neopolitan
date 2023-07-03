use nom::IResult;
use crate::parsers::attribute::Attribute;

pub fn global_attr_class(source: &str) -> IResult<&str, Attribute> {
    Ok((source, Attribute::None))
}


// use crate::parsers::attribute::Attribute;
// use nom::IResult;

// pub fn global_class(source: &str) -> IResult<&str, Attribute> {
//     Ok((source, Attribute::Empty))
// }

// #[cfg(test)]
// mod test {

//     use super::*;
//     use rstest::rstest;

//     #[rstest]
//     #[ignore]
//     #[case("|class: alfa>>", (">>", Attribute::Class(vec!["alfa".to_string()])))]
//     fn solo_class_tester(
//         #[case] input: &str,
//         #[case] expected: (&str, Attribute),
//     ) {
//         assert_eq!(expected, global_class(input).unwrap());
//     }

// }