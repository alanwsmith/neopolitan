use nom::IResult;
use crate::parsers::attribute::Attribute;
use nom::bytes::complete::is_not;
use nom::multi::many0;
use nom::bytes::complete::tag;
use nom::branch::alt;
use nom::character::complete::multispace1;
use nom::multi::separated_list1;
use nom::sequence::terminated;
use nom::character::complete::space1;

pub fn class_attr(source: &str) -> IResult<&str, Attribute> {
    dbg!("---------------------------");

    let (source, values) = 
    terminated(
separated_list1(space1, is_not(" ")),
        tag(">>"))(source)?;
    
dbg!(&values);

dbg!("---------------------------");

    // many0(is_not("|>"))(source)?;
    // dbg!(&source);
    // dbg!(&values);

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