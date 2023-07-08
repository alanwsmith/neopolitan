use crate::section_attrs::sec_attrs;
use crate::sections::alt;
use crate::sections::SecAttr;
use crate::sections::Section;
use nom::bytes::complete::is_not;
use nom::bytes::complete::tag;
use nom::bytes::complete::tag_no_case;
use nom::bytes::complete::take_until;
use nom::character::complete::line_ending;
use nom::character::complete::not_line_ending;
use nom::combinator::opt;
use nom::combinator::rest;
use nom::sequence::delimited;
use nom::sequence::tuple;
use nom::IResult;

pub fn endcode(source: &str) -> IResult<&str, Section> {
    let (source, _) =
        tuple((tag_no_case("-> endcode"), not_line_ending, line_ending))(
            source.trim(),
        )?;
    let (source, _) = alt((take_until("\n\n-> "), rest))(source)?;
    
    Ok((source, Section::None))

    //
}

#[cfg(test)]
mod text {
    use super::*;
    // use crate::blocks::Block;
    use crate::sections::Section;
    // use crate::tags::Tag;
    use crate::section_attrs::SecAttr;
    use rstest::rstest;

    // #[rstest]
    // #[case(
    //     vec!["-> startcode", "", "kick it", "", "-> h2", "", "-> endcode"].join("\n"), 
    //     Section::Code {
    //         attrs: vec![],
    //         text: "kick it\n\n-> h2".to_string()
    //     }
    // )]
    // #[case(
    //     vec!["-> startcode", ">> rust", "", "kick it", "", "-> h2", "", "-> endcode"].join("\n"), 
    //     Section::Code {
    //         attrs: vec![SecAttr::Class(vec!["language-rust".to_string()])],
    //         text: "kick it\n\n-> h2".to_string()
    //     }
    // )]
    // fn solo_code_test(#[case] i: String, #[case] e: Section) {
    //     assert_eq!(e, startcode(i.as_str()).unwrap().1)
    // }
}
