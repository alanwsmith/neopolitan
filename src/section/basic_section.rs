#![allow(unused)]
use super::Section;
use crate::config::Config;
use crate::section::basic_section_full::basic_section_full;
use crate::section_category::SectionCategory;
use crate::section_parent::SectionParent;
use crate::span_strings::space0_line_ending_or_eof::space0_line_ending_or_eof;
use nom::Parser;
use nom::bytes::complete::is_not;
use nom::character::complete::space1;
use nom::sequence::pair;
use nom::sequence::terminated;
use nom::{IResult, branch::alt, bytes::complete::tag, combinator::rest};
use serde::{Deserialize, Serialize};

pub fn basic_section<'a>(
    source: &'a str,
    config: &'a Config,
    parent: &'a SectionParent,
    debug: bool,
) -> IResult<&'a str, Section> {
    let (source, section) = alt((
        // |src| basic_section_start(source, config, parent, debug),
        |src| basic_section_full(source, config, parent, debug),
    ))
    .parse(source)?;
    Ok((source, section))
}

// pub fn basic_section_start<'a>(
//     source: &'a str,
//     config: &'a Config,
//     parent: &'a SectionParent,
//     debug: bool,
// ) -> IResult<&'a str, Section> {
//     let initial_source = source;
//     let (source, _) = pair(tag("--"), space1).parse(source)?;
//     let (source, kind) = terminated(
//         is_not(" /\t\r\n"),
//         pair(tag("/"), space0_line_ending_or_eof),
//     )
//     .parse(source)?;
//     Ok((
//         "",
//         Section {
//             category: SectionCategory::Raw,
//             kind: "asdf".to_string(),
//         },
//     ))
// }
