// use crate::get_raw_sections::*;
// // use crate::get_sections::*;
// use crate::raw_section::RawSection;
// use serde::{Deserialize, Serialize};
// use std::collections::HashMap;

// #[derive(Serialize, Deserialize, Debug, PartialEq)]
// pub struct Page {
//     pub attributes: HashMap<String, String>,
//     pub children: Vec<Section>,
// }

// #[derive(Serialize, Deserialize, Debug, PartialEq)]
// pub enum Section {
//     ATTRIBUTES {
//         raw: HashMap<String, String>,
//     },
//     H1 {
//         attributes: HashMap<String, String>,
//         children: Vec<Content>,
//     },
//     H2 {
//         attributes: HashMap<String, String>,
//         children: Vec<Content>,
//     },
//     H3 {
//         attributes: HashMap<String, String>,
//         children: Vec<Content>,
//     },
//     H4 {
//         attributes: HashMap<String, String>,
//         children: Vec<Content>,
//     },
//     H5 {
//         attributes: HashMap<String, String>,
//         children: Vec<Content>,
//     },
//     H6 {
//         attributes: HashMap<String, String>,
//         children: Vec<Content>,
//     },
//     P {
//         attributes: HashMap<String, String>,
//         children: Vec<Content>,
//     },
//     Title {
//         attributes: HashMap<String, String>,
//         children: Vec<Content>,
//     },
//     PARAGRAPHS {},
// }

// #[derive(Serialize, Deserialize, Debug, PartialEq)]
// pub enum Marker {
//     ATTRIBUTES,
//     H1,
//     H2,
//     H3,
//     H4,
//     H5,
//     H6,
//     PARAGRAPHS,
//     Title,
// }

// #[derive(Serialize, Deserialize, Debug, PartialEq)]
// pub enum Content {
//     Text { value: String },
//     PlainText { value: String },
// }

// pub fn parse(source: &str) -> Page {
//     let raw_sections = get_sections(source).unwrap().1;
//     let mut sections: Vec<Section> = vec![];
//     let mut attrs: HashMap<String, String> = HashMap::new();
//     for raw_section in raw_sections {
//         match raw_section {
//             Section::ATTRIBUTES { raw } => attrs = raw,
//             _ => {
//                 sections.push(raw_section);
//             }
//         }
//     }
//     let page = Page {
//         attributes: attrs,
//         children: sections,
//     };
//     page
// }

// pub fn parse_dev(source: &str) -> Page {
//     let raw_sections: Vec<RawSection> = get_raw_sections(source).unwrap().1;
//     dbg!(raw_sections);
//     // let raw_sections = get_sections_dev(source).unwrap().1;
//     // let mut sections: Vec<Section> = vec![];
//     // let mut attrs: HashMap<String, String> = HashMap::new();
//     // for raw_section in raw_sections {
//     //     match raw_section {
//     //         Section::ATTRIBUTES { raw } => attrs = raw,
//     //         _ => {
//     //             sections.push(raw_section);
//     //         }
//     //     }
//     // }
//     // let page = Page {
//     //     attributes: attrs,
//     //     children: sections,
//     // };
//     let page = Page {
//         attributes: HashMap::new(),
//         children: vec![Section::Title {
//             attributes: HashMap::new(),
//             children: vec![Content::PlainText {
//                 value: "This Is A Title".to_string(),
//             }],
//         }],
//     };
//     page
// }
