#![allow(warnings)]
use crate::attributes::*;
use crate::chunk::Chunk;
use crate::page::Page;
use crate::section::*;
use crate::text::*;
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

pub fn title(source: &str) -> IResult<&str, Section> {
    // dbg!(&source);
    let (remainder, mut attributes) = attributes(source)?;
    // dbg!(&attributes);
    // dbg!(&remainder);
    let (remainder, title) = alt((take_until("\n\n"), rest))(remainder)?;
    let (remainder, _) = multispace0(remainder)?;
    let (remainder, mut paragraph_texts) =
        separated_list0(tag("\n\n"), take_until("\n\n"))(remainder)?;
    if remainder != "" {
        paragraph_texts.push(remainder.trim());
    }
    let mut chunks: Vec<Chunk> = vec![Chunk::H1 {
        // attributes: Some(vec![(Some("class".to_string()), Some("title".to_string()))]),
        attributes,
        children: Some(vec![Chunk::Text {
            attributes: None,
            value: Some(title.to_string()),
        }]),
    }];
    chunks.extend(paragraph_texts.iter().map(|p| Chunk::P {
        attributes: None,
        children: text(p).unwrap().1,
    }));
    let expected = Section::TitleSection {
        attributes: None,
        children: Some(chunks),
    };

    // let expected = Section::TitleSection {
    //     attributes: None,
    //     children: Some(vec![Chunk::H1 {
    //         attributes: None,
    //         children: Some(vec![Chunk::Text {
    //             attributes: None,
    //             value: Some("Alfa Bravo".to_string()),
    //         }]),
    //     }]),
    // };

    Ok(("", expected))

    // dbg!(&attributes);

    // attributes: Some(vec![(Some("class".to_string()), Some("title".to_string()))]),

    // dbg!(&paragraph_texts);

    // for paragraph in paragraphs.iter() {
    //     children.as_mut().unwrap().push(Chunk::P {
    //         attributes: attribute_list.clone(),
    //         children: text(paragraph).unwrap().1,
    //     });
    // }

    // dbg!(title);

    // attributes
    //     .as_mut()
    //     .unwrap()
    //     .push((Some("class".to_string()), Some("title".to_string())));

    // Section::TitleSection {
    //     ref mut children, ..
    // } => {
    //     let (source, _) = space0(source)?;
    //     let (source, _) = line_ending(source)?;
    //     let (source, attributes) = many0(attribute_splitter)(source)?;
    //     let mut attribute_map: HashMap<String, String> =
    //         HashMap::from([("class".to_string(), "title".to_string())]);
    //     for attribute in attributes {
    //         let (remainder, key) = take_until(":")(attribute)?;
    //         let (value, _) = tag(":")(remainder)?;
    //         attribute_map.insert(key.trim().to_string(), value.trim().to_string());
    //     }
    //
    //     let (return_content, content) = alt((take_until("\n-> "), rest))(source)?;
    //     let (remainder, title) = alt((take_until("\n\n"), rest))(content)?;
    //     let (remainder, _) = multispace0(remainder)?;
    //     let (remainder, mut paragraphs) =
    //         separated_list0(tag("\n\n"), take_until("\n\n"))(remainder)?;
    //     paragraphs.push(remainder.trim());
    //     if attribute_map.is_empty() {
    //         children.as_mut().unwrap().push(Chunk::H1 {
    //             attributes: None,
    //             children: Some(vec![Chunk::Text {
    //                 attributes: None,
    //                 value: Some(title.trim().to_string()),
    //             }]),
    //         });
    //     } else {
    //         children.as_mut().unwrap().push(Chunk::H1 {
    //             // attributes: Some(attribute_map),
    //             attributes: None,
    //             children: Some(vec![Chunk::Text {
    //                 attributes: None,
    //                 value: Some(title.trim().to_string()),
    //             }]),
    //         });
    //     }
    //     for paragraph in paragraphs.iter() {
    //         if paragraph.is_empty() {
    //         } else {
    //             children.as_mut().unwrap().push(Chunk::P {
    //                 attributes: None,
    //                 children: Some(vec![Chunk::Text {
    //                     attributes: None,
    //                     value: Some(paragraph.trim().to_string()),
    //                 }]),
    //             });
    //         }
    //     }
    //     Ok((return_content, block))
    // }

    // match attributes {
    //     Some(x) => {
    //         let expected = Section::TitleSection {
    //             attributes: None,
    //             children: Some(vec![Chunk::H1 {
    //                 attributes: None,
    //                 children: Some(vec![Chunk::Text {
    //                     attributes: None,
    //                     value: Some("Alfa Bravo".to_string()),
    //                 }]),
    //             }]),
    //         };
    //         Ok(("", expected))

    //             if x.len() == 1 {
    //                 let response = Section::CodeSection {
    //                     attributes: None,
    //                     language: Some(x[0].0.as_ref().unwrap().to_string()),
    //                     children: Some(vec![Chunk::Text {
    //                         attributes: None,
    //                         value: Some(remainder.to_string()),
    //                     }]),
    //                 };
    //                 Ok(("", response))
    //             } else {
    //                 let response = Section::CodeSection {
    //                     attributes: Some(
    //                         x.clone()
    //                             .into_iter()
    //                             .skip(1)
    //                             .collect::<Vec<(Option<String>, Option<String>)>>(),
    //                     ),
    //                     language: Some(x[0].0.as_ref().unwrap().to_string()),
    //                     children: Some(vec![Chunk::Text {
    //                         attributes: None,
    //                         value: Some(remainder.to_string()),
    //                     }]),
    //                 };
    //                 Ok(("", response))
    //             }
    // }
    // None => {

    // let mut chunks: Vec<Chunk> = vec![Chunk::H1 {
    //     // attributes: Some(vec![(Some("class".to_string()), Some("title".to_string()))]),
    //     attributes: None,
    //     children: Some(vec![Chunk::Text {
    //         attributes: None,
    //         value: Some(title.to_string()),
    //     }]),
    // }];
    // chunks.extend(paragraph_texts.iter().map(|p| Chunk::P {
    //     attributes: None,
    //     children: text(p).unwrap().1,
    // }));
    // let expected = Section::TitleSection {
    //     attributes: None,
    //     children: Some(chunks),
    // };

    // let expected = Section::TitleSection {
    //     attributes: None,
    //     children: Some(vec![Chunk::H1 {
    //         attributes: Some(vec![(Some("class".to_string()), Some("title".to_string()))]),
    //         children: Some(vec![Chunk::Text {
    //             attributes: None,
    //             value: Some(title.to_string()),
    //         }]),
    //     }]),
    // };

    // let expected = Section::TitleSection {
    //     attributes: None,
    //     children: Some(vec![
    //         Chunk::H1 {
    //             attributes: Some(vec![(
    //                 Some("class".to_string()),
    //                 Some("title".to_string()),
    //             )]),
    //             children: Some(vec![Chunk::Text {
    //                 attributes: None,
    //                 value: Some(title.to_string()),
    //             }]),
    //         },
    //         Chunk::P {
    //             attributes: None,
    //             children: Some(vec![Chunk::Text {
    //                 attributes: None,
    //                 value: Some("Charlie delta echo".to_string()),
    //             }]),
    //         },
    //         Chunk::P {
    //             attributes: None,
    //             children: Some(vec![Chunk::Text {
    //                 attributes: None,
    //                 value: Some("Foxtrot golf hotel".to_string()),
    //             }]),
    //         },
    //     ]),
    // };

    // Ok(("", expected))

    //             let response = Section::CodeSection {
    //                 attributes: None,
    //                 language: None,
    //                 children: Some(vec![Chunk::Text {
    //                     attributes: None,
    //                     value: Some(remainder.to_string()),
    //                 }]),
    //             };
    //             Ok(("", response))
    //         }
    // } // Ok(("", expected))
    // }
}
