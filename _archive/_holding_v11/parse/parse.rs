#![allow(warnings)]
// use crate::block::block::*;
use crate::section::section::*;
// use crate::snippet::snippet::*;
use nom::character::complete::multispace0;
use nom::character::complete::none_of;
use nom::character::complete::one_of;
use nom::combinator::eof;
use nom::combinator::not;
use nom::error::Error;
use nom::multi::many_till;
use nom::IResult;

pub fn parse(source: &str) -> IResult<&str, Option<Vec<Section>>> {
    // do a check to see if it starts with a section
    // let (a, _) = multispace0(source)?;

    // let x = none_of::<&str, &str, Error<&str>>("-")(a);
    //match x {
    //Err => None,
    // Ok() => {
    let (_, sections) = many_till(section, eof)(source)?;
    Ok(("", Some(sections.0)))
    // }
    // }
    // dbg!(a);
    // dbg!(b);
    //dbg!(x);
}

// pub fn parse_dev(source: &str) -> IResult<&str, Option<Vec<Section>>> {
//     let (_, sections) = many_till(section, eof)(source)?;
//     Ok(("", Some(sections.0)))
//     // dbg!(sections);
//     // Ok((
//     //     "",
//     //     Some(vec![Section::TitleSection {
//     //         attributes: Some(vec![(Some("id".to_string()), Some("bravo".to_string()))]),
//     //         children: Some(vec![Block::Text {
//     //             snippets: Some(vec![Snippet::Plain {
//     //                 text: Some("Set The Piece".to_string()),
//     //             }]),
//     //         }]),
//     //     }]),
//     // ))
// }

// #[cfg(test)]
// mod test {
//     use super::*;
//     #[test]
//     pub fn get_nothing_if_it_doesnt_start_with_a_tag() {
//         let source = "-> title\n\nThis is a thing";
//         let results = parse(source).unwrap().1;
//         assert_eq!(None, results);
//     }
// }
