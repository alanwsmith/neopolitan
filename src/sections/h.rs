use crate::blocks::headline::headline;
use crate::blocks::paragraph::paragraph;
use crate::section_attrs::sec_attrs;
use crate::sections::Section;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::bytes::complete::take_until;
use nom::character::complete::line_ending;
use nom::character::complete::not_line_ending;
use nom::combinator::eof;
use nom::combinator::rest;
use nom::multi::many_till;
use nom::sequence::delimited;
use nom::sequence::pair;
use nom::IResult;

pub fn h(source: &str) -> IResult<&str, Section> {
    let (source, level) = delimited(
        tag("-> "),
        alt((
            tag("h1"),
            tag("h2"),
            tag("h3"),
            tag("h4"),
            tag("h5"),
            tag("h6"),
        )),
        pair(not_line_ending, line_ending),
    )(source.trim())?;
    let (source, content) = alt((take_until("\n\n->"), rest))(source.trim())?;
    let (content, attrs) = sec_attrs(content.trim())?;
    let (content, headline) = headline(content.trim())?;
    let (_, paragraphs) = many_till(paragraph, eof)(content.trim())?;

    match level {
        "h1" => Ok((
            source,
            Section::H1 {
                attrs,
                headline,
                paragraphs: paragraphs.0,
            },
        )),
        "h2" => Ok((
            source,
            Section::H2 {
                attrs,
                headline,
                paragraphs: paragraphs.0,
            },
        )),
        "h3" => Ok((
            source,
            Section::H3 {
                attrs,
                headline,
                paragraphs: paragraphs.0,
            },
        )),
        "h4" => Ok((
            source,
            Section::H4 {
                attrs,
                headline,
                paragraphs: paragraphs.0,
            },
        )),
        "h5" => Ok((
            source,
            Section::H5 {
                attrs,
                headline,
                paragraphs: paragraphs.0,
            },
        )),
        "h6" => Ok((
            source,
            Section::H6 {
                attrs,
                headline,
                paragraphs: paragraphs.0,
            },
        )),
        _ => panic!("error with h section"),
    }
}

// #[cfg(test)]

// mod test {
//     use super::*;
//     use crate::blocks::Block;
//     use crate::sections::Section;
//     use crate::tags::Tag;

//     #[test]
//     pub fn basic_title() {
//         let lines = vec!["-> title", "", "alfa bravo"].join("\n");
//         let expected = Section::Title {
//             attrs: vec![],
//             headline: Block::Headline {
//                 snippets: vec![Tag::Text {
//                     text: "alfa bravo".to_string(),
//                 }],
//             },
//             paragraphs: vec![],
//         };
//         assert_eq!(expected, title(lines.as_str()).unwrap().1);
//     }

//     #[test]
//     pub fn title_with_two_lines() {
//         let lines =
//             vec!["-> title", "", "charlie delta", "echo foxtrot"].join("\n");
//         let expected = Section::Title {
//             attrs: vec![],
//             headline: Block::Headline {
//                 snippets: vec![Tag::Text {
//                     text: "charlie delta echo foxtrot".to_string(),
//                 }],
//             },
//             paragraphs: vec![],
//         };
//         assert_eq!(expected, title(lines.as_str()).unwrap().1);
//     }

//     #[test]
//     pub fn title_with_paragraphs() {
//         let lines = vec![
//             "-> title",
//             "",
//             "golf hotel",
//             "whiskey tango",
//             "",
//             "foxtrot alfa",
//         ]
//         .join("\n");
//         let expected = Section::Title {
//             attrs: vec![],
//             headline: Block::Headline {
//                 snippets: vec![Tag::Text {
//                     text: "golf hotel whiskey tango".to_string(),
//                 }],
//             },
//             paragraphs: vec![Block::Paragraph {
//                 snippets: vec![Tag::Text {
//                     text: "foxtrot alfa".to_string(),
//                 }],
//             }],
//         };
//         assert_eq!(expected, title(lines.as_str()).unwrap().1);
//     }
// }
