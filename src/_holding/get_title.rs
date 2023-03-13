// use crate::get_text::*;
// use crate::parse::Section;
// use nom::IResult;
// use std::collections::HashMap;

// pub fn get_title(source: &str) -> IResult<&str, Section> {
//     Ok((
//         "",
//         Section::Title {
//             attributes: HashMap::new(),
//             children: vec![get_text(source).unwrap().1],
//         },
//     ))
// }
