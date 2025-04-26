use crate::config::Config;
use crate::section_metadata::RawSectionMetaData;
use crate::section_parent::SectionParent;
use nom::IResult;
use nom::Parser;
use nom::branch::alt;
use nom::bytes::complete::is_not;
use nom::bytes::complete::tag;
use nom::character::complete::line_ending;
use nom::character::complete::multispace1;
use nom::character::complete::space0;
use nom::character::complete::space1;
use nom::combinator::eof;
use nom::combinator::not;
use nom::multi::many1;
use nom::sequence::pair;

pub fn raw_section_flag<'a>(
    source: &'a str,
    _config: &'a Config,
    _parent: &'a SectionParent,
    _debug: bool,
) -> IResult<&'a str, RawSectionMetaData> {
    let (source, _) = tag("--").parse(source)?;
    let (source, _) = space1.parse(source)?;
    let (source, parts) = many1(alt((
        is_not(": \n\t\\"),
        pair(tag(":"), not(alt((multispace1, eof)))).map(|x| x.0),
    )))
    .parse(source)?;
    let (source, _) =
        alt((pair(space0, line_ending), pair(space0, eof))).parse(source)?;
    let flag = RawSectionMetaData::Flag {
        string: parts
            .iter()
            .map(|part| part.to_string())
            .collect::<Vec<String>>()
            .join("")
            .to_string(),
    };
    Ok((source, flag))
}

#[cfg(test)]
mod test {
    use super::*;
    use pretty_assertions::assert_eq;
    use rstest::rstest;

    #[rstest]
    #[case("alfa")]
    #[case("bravo-charlie")]
    #[case("https://www.example.com/")]
    #[case("  leading_spaces_are_okay")]
    #[case("trailing_spaces_are_okay   ")]
    #[case("these_characters_are_okay:!@#$%^&*()[]<>|-")]
    fn raw_section_flag_valid_tests(#[case] left: &str) {
        let config = &Config::default();
        let parent = &SectionParent::Basic;
        let debug = false;
        let source = format!("-- {}", left);
        let left = RawSectionMetaData::Flag {
            string: left.trim().to_string(),
        };
        let right = raw_section_flag(&source, config, parent, debug).unwrap().1;
        assert_eq!(left, right);
    }

    #[rstest]
    #[case("no spaces")]
    #[case("alfa:")]
    #[case("bravo: charlie")]
    #[case("delta: ")]
    #[case("no_escaped_\\allowed")]
    fn raw_section_flag_invalid_tests(#[case] left: &str) {
        let config = &Config::default();
        let parent = &SectionParent::Basic;
        let debug = false;
        let source = format!("-- {}", left);
        let right = raw_section_flag(&source, config, parent, debug);
        if right.is_err() {
            assert!(true)
        } else {
            dbg!(left);
            assert!(false)
        }
    }
}
