#![allow(unused)]
use crate::neo_config::NeoConfig;
use crate::section_attribute::SectionAttribute;
use crate::section_attribute::section_attribute_line;
use crate::section_bound::SectionBound;
use crate::section_category::SectionCategory;
use crate::section_flag::section_flag;
use crate::section_parent::SectionParent;
use crate::span_strings::space0_line_ending_or_eof::space0_line_ending_or_eof;
use nom::Parser;
use nom::bytes::complete::is_not;
use nom::character::complete::space1;
use nom::multi::many0;
use nom::sequence::pair;
use nom::sequence::terminated;
use nom::{IResult, branch::alt, bytes::complete::tag, combinator::rest};
use serde::{Deserialize, Serialize};

pub fn section_metadata<'a>(
    source: &'a str,
    config: &'a NeoConfig,
    parent: &'a SectionParent,
    debug: bool,
) -> IResult<&'a str, (Vec<SectionAttribute>, Vec<String>)> {
    // let (source, raw_metadata) =
    //     alt((|src| section_flag(src, config, parent, debug),)).parse(source)?;
    Ok((source, (vec![], vec![])))
}

#[cfg(test)]
mod test {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    #[ignore]
    pub fn section_metadata_basic_test() {
        let config = &NeoConfig::default();
        let source = "-- test-flag\n-- alfa: bravo\n\n";
        let parent = &SectionParent::Basic;
        let debug = false;
        let left = (vec![], vec!["test-flag".to_string()]);
        let right = section_metadata(source, config, parent, debug).unwrap().1;
        assert_eq!(left, right);
    }
}
