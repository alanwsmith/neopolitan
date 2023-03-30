#![allow(warnings)]
use crate::attributes::*;
use crate::blurb::*;
use crate::categories::*;
use crate::chunk::Chunk;
use crate::code::*;
use crate::h1::*;
use crate::h2::*;
use crate::h3::*;
use crate::h4::*;
use crate::h5::*;
use crate::h6::*;
use crate::hr::*;
use crate::image::image;
use crate::list::*;
use crate::note::*;
use crate::text::*;
use crate::title::*;
use crate::video::*;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::bytes::complete::tag_no_case;
use nom::bytes::complete::take_until;
use nom::character::complete::line_ending;
use nom::character::complete::multispace0;
use nom::character::complete::not_line_ending;
use nom::character::complete::space0;
use nom::combinator::rest;
use nom::multi::many0;
use nom::multi::separated_list0;
use nom::IResult;
use nom::Parser;
use serde::Deserialize;
use serde::Serialize;
use std::collections::HashMap;

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
#[serde(tag = "type")]
pub enum Section {
    AttributesSection {
        children: Option<Vec<Chunk>>,
    },
    BlurbSection {
        children: Option<Vec<Chunk>>,
    },
    CategoriesSection {
        children: Option<Vec<Chunk>>,
    },
    CodeSection {
        language: Option<String>,
        attributes: Option<HashMap<String, Option<String>>>,
        children: Option<Vec<Chunk>>,
    },
    H1Section {
        attributes: Option<HashMap<String, Option<String>>>,
        children: Option<Vec<Chunk>>,
    },
    H2Section {
        attributes: Option<HashMap<String, Option<String>>>,
        children: Option<Vec<Chunk>>,
    },
    H3Section {
        attributes: Option<HashMap<String, Option<String>>>,
        children: Option<Vec<Chunk>>,
    },
    H4Section {
        attributes: Option<HashMap<String, Option<String>>>,
        children: Option<Vec<Chunk>>,
    },
    H5Section {
        attributes: Option<HashMap<String, Option<String>>>,
        children: Option<Vec<Chunk>>,
    },
    H6Section {
        attributes: Option<HashMap<String, Option<String>>>,
        children: Option<Vec<Chunk>>,
    },
    HRSection {
        attributes: Option<HashMap<String, Option<String>>>,
    },
    ImageSection {
        attributes: Option<HashMap<String, Option<String>>>,
    },
    ListSection {
        attributes: Option<HashMap<String, Option<String>>>,
        children: Option<Vec<Chunk>>,
    },
    OrderedListSection {
        attributes: Option<HashMap<String, Option<String>>>,
        children: Option<Vec<Chunk>>,
    },
    NoteSection {
        attributes: Option<HashMap<String, Option<String>>>,
        children: Option<Vec<Chunk>>,
    },
    ParagraphSection {
        attributes: Option<HashMap<String, Option<String>>>,
        children: Option<Vec<Chunk>>,
    },
    TitleSection {
        attributes: Option<HashMap<String, Option<String>>>,
        children: Option<Vec<Chunk>>,
    },
    VideoSection {
        attributes: Option<HashMap<String, Option<String>>>,
    },
    Placeholder,
}

fn attribute_splitter(source: &str) -> IResult<&str, &str> {
    let (source, _) = multispace0(source)?;
    let (source, _) = tag(">> ")(source)?;
    let (source, value) = not_line_ending(source)?;
    let (source, _) = multispace0(source)?;
    Ok((source, value))
}

pub fn section(source: &str) -> IResult<&str, Section> {
    let (source, _) = multispace0(source)?;
    let (source, mut block) = alt((
        tag_no_case("-> attributes").map(|_| Section::AttributesSection { children: None }),
        tag_no_case("-> blurb").map(|_| Section::BlurbSection { children: None }),
        tag_no_case("-> CODE").map(|_| Section::CodeSection {
            attributes: None,
            language: None,
            children: Some(vec![]),
        }),
        tag_no_case("-> HR").map(|_| Section::HRSection { attributes: None }),
        tag_no_case("-> H1").map(|_| Section::H1Section {
            children: None,
            attributes: None,
        }),
        tag_no_case("-> H2").map(|_| Section::H2Section {
            children: None,
            attributes: None,
        }),
        tag_no_case("-> H3").map(|_| Section::H3Section {
            children: None,
            attributes: None,
        }),
        tag_no_case("-> H4").map(|_| Section::H4Section {
            children: None,
            attributes: None,
        }),
        tag_no_case("-> H5").map(|_| Section::H5Section {
            children: None,
            attributes: None,
        }),
        tag_no_case("-> H6").map(|_| Section::H6Section {
            children: None,
            attributes: None,
        }),
        tag_no_case("-> image").map(|_| Section::ImageSection { attributes: None }),
        // TODO:
        tag_no_case("-> NOTE").map(|_| Section::NoteSection {
            attributes: None,
            children: None,
        }),
        tag_no_case("-> P").map(|_| Section::ParagraphSection {
            attributes: None,
            children: Some(vec![]),
        }),
        tag_no_case("-> TITLE").map(|_| Section::TitleSection {
            attributes: None,
            children: Some(vec![]),
        }),
        // TODO:
        tag_no_case("-> LIST").map(|_| Section::ListSection {
            attributes: None,
            children: None,
        }),
        // TODO:
        tag_no_case("-> OLIST").map(|_| Section::OrderedListSection {
            attributes: None,
            children: None,
        }),
        // TODO:
        tag_no_case("-> categories").map(|_| Section::CategoriesSection { children: None }),
        // TODO:
        tag_no_case("-> video").map(|_| Section::VideoSection { attributes: None }),
    ))(source)?;
    match block {
        // Section::TitleSection {
        //     ref mut children, ..
        // } => {
        //     let (source, attribute_list) = attributes(source)?;
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
        Section::BlurbSection { .. } => {
            let (return_content, source) = alt((take_until("\n-> "), rest))(source)?;
            let (_, block) = blurb(source)?;
            Ok((return_content, block))
        }
        Section::CategoriesSection { .. } => {
            let (return_content, source) = alt((take_until("\n-> "), rest))(source)?;
            let (_, block) = categories(source)?;
            Ok((return_content, block))
        }
        Section::CodeSection {
            children: _children,
            attributes: _attributes,
            language: _language,
        } => {
            let (return_content, source) = alt((take_until("\n-> "), rest))(source)?;
            let (_, block) = code(source)?;
            Ok((return_content, block))
        }
        Section::H1Section { .. } => {
            let (return_content, source) = alt((take_until("\n-> "), rest))(source)?;
            let (_, block) = h1(source)?;
            Ok((return_content, block))
        }
        Section::H2Section { .. } => {
            let (return_content, source) = alt((take_until("\n-> "), rest))(source)?;
            let (_, block) = h2(source)?;
            Ok((return_content, block))
        }
        Section::H3Section { .. } => {
            let (return_content, source) = alt((take_until("\n-> "), rest))(source)?;
            let (_, block) = h3(source)?;
            Ok((return_content, block))
        }
        Section::H4Section { .. } => {
            let (return_content, source) = alt((take_until("\n-> "), rest))(source)?;
            let (_, block) = h4(source)?;
            Ok((return_content, block))
        }
        Section::H5Section { .. } => {
            let (return_content, source) = alt((take_until("\n-> "), rest))(source)?;
            let (_, block) = h5(source)?;
            Ok((return_content, block))
        }
        Section::H6Section { .. } => {
            let (return_content, source) = alt((take_until("\n-> "), rest))(source)?;
            let (_, block) = h6(source)?;
            Ok((return_content, block))
        }
        Section::HRSection { .. } => {
            let (return_content, source) = alt((take_until("\n-> "), rest))(source)?;
            let (_, block) = hr(source)?;
            Ok((return_content, block))
        }
        Section::ImageSection { .. } => {
            let (return_content, source) = alt((take_until("\n-> "), rest))(source)?;
            let (_, block) = image(source)?;
            Ok((return_content, block))
        }
        Section::ListSection {
            children: _children,
            attributes: _attributes,
        } => {
            let (return_content, source) = alt((take_until("\n-> "), rest))(source)?;
            let (_, block) = list(source)?;
            Ok((return_content, block))
        }
        Section::NoteSection {
            children: _children,
            attributes: _attributes,
        } => {
            let (return_content, source) = alt((take_until("\n-> "), rest))(source)?;
            let (_, block) = note(source)?;
            Ok((return_content, block))
        }
        Section::OrderedListSection {
            children: _children,
            attributes: _attributes,
        } => {
            let (return_content, source) = alt((take_until("\n-> "), rest))(source)?;
            let (_, block) = list(source)?;
            Ok((return_content, block))
        }
        Section::ParagraphSection {
            ref mut children, ..
        } => {
            let (source, attributes) = attributes(source)?;
            let (return_content, content) = alt((take_until("\n-> "), rest))(source)?;
            let (content, _) = multispace0(content)?;
            let (remainder, mut paragraphs) =
                separated_list0(tag("\n\n"), take_until("\n\n"))(content)?;
            paragraphs.push(remainder);
            for paragraph in paragraphs.iter() {
                children.as_mut().unwrap().push(Chunk::P {
                    attributes: attributes.clone(),
                    children: text(paragraph).unwrap().1,
                });
            }
            Ok((return_content, block))
        }
        Section::TitleSection { .. } => {
            let (return_content, source) = alt((take_until("\n-> "), rest))(source)?;
            let (_, block) = title(source)?;
            Ok((return_content, block))
        }
        Section::VideoSection {
            attributes: _attributes,
        } => {
            let (return_content, source) = alt((take_until("\n-> "), rest))(source)?;
            let (_, block) = video(source)?;
            Ok((return_content, block))
        }
        _ => {
            let block = Section::Placeholder;
            Ok(("", block))
        }
    }
}

// use crate::chunk::Chunk;
// use serde::Deserialize;
// use serde::Serialize;

// #[derive(Debug, PartialEq, Serialize, Deserialize)]
// #[serde(tag = "type")]
// pub enum Section {
//     TitleSection {
//         attributes: Option<Vec<(Option<String>, Option<String>)>>,
//         children: Option<Vec<Chunk>>,
//     },
//     ParagraphSection {
//         attributes: Option<Vec<(Option<String>, Option<String>)>>,
//         children: Option<Vec<Chunk>>,
//     },
// }
