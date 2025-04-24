use crate::section_attrs::sec_attrs;
use crate::sections::alt;
use crate::sections::Section;
use nom::bytes::complete::tag_no_case;
use nom::bytes::complete::take_until;
use nom::character::complete::line_ending;
use nom::character::complete::not_line_ending;
use nom::combinator::rest;
use nom::sequence::tuple;
use nom::IResult;

pub fn opendiv(source: &str) -> IResult<&str, Section> {
    let (source, _) =
        tuple((tag_no_case("-> opendiv"), not_line_ending, line_ending))(
            source.trim(),
        )?;
    let (source, content) = alt((take_until("\n\n->"), rest))(source.trim())?;
    let (_, attrs) = sec_attrs(content.trim())?;
    Ok((source, Section::OpenDiv { attrs }))
}

#[cfg(test)]
mod text {
    use super::*;
    use crate::section_attrs::SecAttr;
    use crate::sections::Section;
    use rstest::rstest;

    #[rstest]
    #[case(
        vec!["-> opendiv", ">> class: alfa", ""].join("\n"),
        Section::OpenDiv {
            attrs: vec![SecAttr::Class(vec!["alfa".to_string()])],
        }
    )]
    fn opendiv_test(#[case] i: String, #[case] e: Section) {
        assert_eq!(e, opendiv(i.as_str()).unwrap().1)
    }
}
