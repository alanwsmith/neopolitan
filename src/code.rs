#![allow(warnings)]
use crate::attributes::*;
use crate::chunk::Chunk;
use crate::page::Page;
use crate::process_text::*;
use crate::section::*;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::bytes::complete::take_until;
use nom::character::complete::char;
use nom::character::complete::line_ending;
use nom::character::complete::multispace0;
use nom::character::complete::multispace1;
use nom::character::complete::not_line_ending;
use nom::character::complete::space0;
use nom::combinator::eof;
use nom::combinator::not;
use nom::combinator::rest;
use nom::error::Error;
use nom::error::ErrorKind;
use nom::multi::many0;
use nom::multi::many1;
use nom::multi::many_till;
use nom::multi::separated_list0;
use nom::multi::separated_list1;
use nom::sequence::delimited;
use nom::sequence::pair;
use nom::sequence::preceded;
use nom::sequence::tuple;
use nom::Err;
use nom::IResult;
use nom::Parser;
use std::collections::HashMap;

// pub fn code(source: &str) -> IResult<&str, Section> {
//     let (source, value) = multispace0(source)?;
//     let (source, value) = separated_list0(tag(">> "), take_until(">> "))(source)?;
//     if value.is_empty() {
//         let response = Section::CodeSection {
//             attributes: None,
//             language: None,
//             children: vec![Chunk::Text {
//                 value: source.trim().to_string(),
//             }],
//         };
//         Ok(("", response))
//     } else {
//         let (source, language) = tag(">> ")(source)?;
//         let (source, language) = not_line_ending(source)?;
//         let response = Section::CodeSection {
//             attributes: None,
//             language: Some(language.to_string()),
//             children: vec![Chunk::Text {
//                 value: source.trim().to_string(),
//             }],
//         };
//         Ok(("", response))
//     }
// }

pub fn code(source: &str) -> IResult<&str, Section> {
    dbg!(&source);
    let (remainder, attributes) = attributes(source)?;
    match attributes {
        Some(x) => {
            if x.len() == 1 {
                let response = Section::CodeSectionDev {
                    attributes: None,
                    language: Some(x[0].0.as_ref().unwrap().to_string()),
                    children: vec![Chunk::Text {
                        value: remainder.to_string(),
                    }],
                };
                Ok(("", response))
            } else {
                let response = Section::CodeSectionDev {
                    attributes: Some(
                        x.clone()
                            .into_iter()
                            .skip(1)
                            .collect::<Vec<(Option<String>, Option<String>)>>(),
                    ),
                    language: Some(x[0].0.as_ref().unwrap().to_string()),
                    children: vec![Chunk::Text {
                        value: remainder.to_string(),
                    }],
                };
                Ok(("", response))
            }
        }
        None => {
            let response = Section::CodeSectionDev {
                attributes: None,
                language: None,
                children: vec![Chunk::Text {
                    value: remainder.to_string(),
                }],
            };
            Ok(("", response))
        }
    }
}

// pub fn code_dev3(source: &str) -> IResult<&str, Section> {
//     let (remainder, attributes) = attributes(source)?;
//     if attributes.clone().unwrap().len() == 1 {
//         let response = Section::CodeSectionDev {
//             attributes: None,
//             language: Some(
//                 attributes.clone().unwrap()[0]
//                     .0
//                     .as_ref()
//                     .unwrap()
//                     .to_string(),
//             ),
//             children: vec![Chunk::Text {
//                 value: remainder.to_string(),
//             }],
//         };
//         Ok(("", response))
//     } else {
//         dbg!(&attributes);
//         dbg!(&remainder);
//         // let attribute_list: Vec<(Option<String>, Option<String>)> = vec![];
//         let response = Section::CodeSectionDev {
//             attributes: Some(
//                 attributes
//                     .clone()
//                     .unwrap()
//                     .into_iter()
//                     .skip(1)
//                     .collect::<Vec<(Option<String>, Option<String>)>>(),
//             ),
//             language: Some(
//                 attributes.clone().unwrap()[0]
//                     .0
//                     .as_ref()
//                     .unwrap()
//                     .to_string(),
//             ),
//             children: vec![Chunk::Text {
//                 value: remainder.to_string(),
//             }],
//         };
//         Ok(("", response))
//     }
// }

// pub fn code_dev2(source: &str) -> IResult<&str, Section> {
//     let (remainder, attributes) = attributes(source)?;
//     let attribute_list: Vec<(Option<String>, Option<String>)> = vec![];
//     let response = Section::CodeSectionDev {
//         attributes: Some(
//             attributes
//                 .clone()
//                 .unwrap()
//                 .into_iter()
//                 .skip(1)
//                 .collect::<Vec<(Option<String>, Option<String>)>>(),
//         ),
//         language: Some(
//             attributes.clone().unwrap()[0]
//                 .0
//                 .as_ref()
//                 .unwrap()
//                 .to_string(),
//         ),
//         children: vec![Chunk::Text {
//             value: remainder.to_string(),
//         }],
//     };
//     Ok(("", response))
// }

// pub fn code_dev(source: &str) -> IResult<&str, Section> {
//     // dbg!(source);
//     let (source, value) = multispace0(source)?;
//     let (source, value) = separated_list0(tag(">> "), take_until(">> "))(source)?;
//     // dbg!(&value);
//     let mut language: Option<String> = None;
//     if value.len() == 2 {
//         // language = Some(value[1].to_string());
//         // dbg!(source);
//         // // let (source, language) = tag(">> ")(source)?;
//         // // let (source, language) = not_line_ending(source)?;
//         // let response = Section::CodeSection {
//         //     attributes: None,
//         //     language,
//         //     children: vec![Chunk::Text {
//         //         value: source.trim().to_string(),
//         //     }],
//         // };
//         Ok(("", Section::Placeholder))
//     } else if value.len() > 2 {
//         language = Some(value[1].trim().to_string());
//         let mut attributes: HashMap<String, String> =
//             HashMap::from([("fence".to_string(), "stone".to_string())]);
//         for attribute in &value {
//             let (value, key) = separated_list1(tag(":"), take_until(":"))(source)?;
//             attributes.insert(key[0].trim().to_string(), value.trim().to_string());
//         }
//         let response = Section::CodeSection {
//             attributes: Some(HashMap::from([
//                 ("fence".to_string(), "stone".to_string()),
//                 ("air".to_string(), "frosty".to_string()),
//             ])),
//             language,
//             children: vec![Chunk::Text {
//                 value: "Two blue fish".to_string(),
//             }],
//         };
//         Ok(("", response))
//     } else {
//         let response = Section::CodeSection {
//             attributes: None,
//             language: None,
//             children: vec![Chunk::Text {
//                 value: source.trim().to_string(),
//             }],
//         };
//         // Ok(("", response))
//         Ok(("", Section::Placeholder))
//     }
//     // if value.is_empty() {
//     //     let response = Section::CodeSection {
//     //         attributes: None,
//     //         language: None,
//     //         children: vec![Chunk::Text {
//     //             value: source.trim().to_string(),
//     //         }],
//     //     };
//     // //         Ok(("", response))
//     // } else {
//     //     dbg!(source);
//     //     let (source, language) = tag(">> ")(source)?;
//     //     let (source, language) = not_line_ending(source)?;
//     //     let response = Section::CodeSection {
//     //         attributes: None,
//     //         language: Some(language.to_string()),
//     //         children: vec![Chunk::Text {
//     //             value: source.trim().to_string(),
//     //         }],
//     //     };
//     //     //       Ok(("", response))
//     // }
// }
