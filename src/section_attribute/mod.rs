use crate::neo_config::NeoConfig;
use crate::section_parent::SectionParent;
use crate::span::Span;
use nom::Parser;
use nom::character::complete::alphanumeric1;
use nom::character::complete::space1;
use nom::multi::many1;
use nom::{IResult, branch::alt, bytes::complete::tag};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub struct SectionAttribute {
    key: String,
}

#[derive(Debug, PartialEq)]
pub struct SectionAttributeLine {
    key: String,
    spans: Vec<Span>,
}

pub fn section_attribute_line<'a>(
    source: &'a str,
    _config: &'a NeoConfig,
    _parent: &'a SectionParent,
    _debug: bool,
) -> IResult<&'a str, SectionAttributeLine> {
    let (source, _) = tag("--").parse(source)?;
    let (source, _) = space1.parse(source)?;
    // TODO: Figure out how this works with
    // accent characters.
    let (source, key_parts) =
        many1(alt((alphanumeric1, tag("-"), tag("_")))).parse(source)?;
    Ok((
        "",
        SectionAttributeLine {
            key: key_parts.join("").to_string(),
            spans: vec![],
        },
    ))
}

#[cfg(test)]
mod test {
    use super::*;
    use pretty_assertions::assert_eq;
    // use rstest::rstest;

    #[test]
    fn section_attribute_line_basic_test() {
        let config = &NeoConfig::default();
        let parent = &SectionParent::Basic;
        let debug = false;
        let source = "-- alfa: bravo";
        let left = SectionAttributeLine {
            key: "alfa".to_string(),
            spans: vec![Span::Text {
                kind: "text".to_string(),
                text: "bravo".to_string(),
            }],
        };
        let right = section_attribute_line(source, config, parent, debug)
            .unwrap()
            .1;
        assert_eq!(left, right);
    }

    // #[rstest]
    // #[case("-- alfa: bravo", "")]
    // #[case("-- key-hyphens-are-okay: alfa", "")]
    // #[case("-- alfa: value-hyphens-are-okay", "")]
    // #[case("-- key_underscores_are_okay: alfa", "")]
    // #[case("-- alfa: value_underscores_are_okay", "")]
    // #[case("--    leading_key_spaces_are_okay: alfa", "")]
    // #[case("-- alfa:   leading_value_spaces_are_okay", "")]
    // #[case("-- alfa: ``can contain code, etc...``", "")]
    // #[case("-- alfa: bravo\n-- charlie: delta", "-- charlie: delta")]
    // fn valid_cases_with_remainders(
    //     #[case] source: &str,
    //     #[case] remainder: &str,
    // ) {
    //     let config = &NeoConfig::default();
    //     let parent = &SectionParent::Basic;
    //     let debug = false;
    //     let right = section_attribute(source, config, parent, debug).unwrap().1;
    //     // match results {
    //     //     Ok(result) => {
    //     //         assert_eq!(result.0, remainder);
    //     //     }
    //     //     Err(e) => {
    //     //         assert!(false, "{}", e)
    //     //     }
    //     // }
    // }

    // #[rstest]
    // #[case("-- alfa")]
    // #[case("-- alfa*bravo: charlie")]
    // #[case("-- https://www.example.com/")]
    // #[case("-- key-must-connect-to-colon : alfa")]
    // fn test_invalid_cases(#[case] source: &str) {
    //     let config = &NeoConfig::default();
    //     let parent = &SectionParent::Basic;
    //     let debug = false;
    //     let results = section_attribute(source, config, parent, debug);
    //     match results {
    //         Ok(_) => {
    //             assert!(
    //                 false,
    //                 "This should not have parsed but it did: {}",
    //                 source
    //             );
    //         }
    //         Err(_) => {
    //             assert!(true);
    //         }
    //     }
    // }

    // // NOTE: Not worrying about trailing spaces right now
    // // Something possible to look at in the future
    // #[rstest]
    // //#[case("-- alfa: bravo charlie ")]
    // // #[case("-- alfa: bravo charlie\n")]
    // // #[case("-- alfa: bravo charlie \n")]
    // fn solo_section_key_value_attr_make_sure_ending_space_is_trimmed(#[case] source: &str) {
    //     let left = (
    //         "",
    //         SectionAttrV42::KeyValueV42(SectionKeyValueAttrV42 {
    //             key: "alfa".to_string(),
    //             value: vec![SpanV42::PlainText(plaintext::PlainTextSpan {
    //                 kind: "plain-text".to_string(),
    //                 text: "bravo charlie".to_string(),
    //             })],
    //         }),
    //     );
    //     let right = section_key_value_attr_v42(source).unwrap();
    //     assert_eq!(left, right);
    // }

    // #[rstest]
    // #[case("alfa", "bravo")]
    // #[case("charlie", "delta-echo")]
    // #[case("  leading_spaces", "are_ok")]
    // #[case("test_value", "    value_leading_spaces_are_ok")]
    // #[case("test_value", "value_trailing_spaces_are_ok    ")]
    // // #[case("test_value", "<<link|named_span|https://www.example.com/>>")]
    // // #[case("test_value", "``code shorthand``")]
    // // #[case("test_value", "``code shorthand|with_flag``")]
    // // TODO: do a character check of all the stuff that should work

    // // #[case("https://www.example.com/")]
    // // #[case("  leading_spaces_are_okay")]
    // // #[case("trailing_spaces_are_okay   ")]
    // // #[case("these_characters_are_okay:!@#$%^&*()[]<>|-")]
    // fn section_flag_attr_v42_valid_flags(#[case] left_key: &str, #[case] left_value: &str) {
    //     let source = format!("-- {}: {}", left_key, left_value);
    //     let response = section_key_value_attr_v42(source.as_str()).unwrap();
    //     let right = match response.1 {
    //         // NOTE: Only checking keys here which means the
    //         // parsing worked. Specific value checks are done
    //         // in other tests
    //         SectionAttrV42::KeyValueV42(data) => data.key,
    //         _ => "".to_string(),
    //     };
    //     assert_eq!(left_key.trim(), right);
    // }

    // Add to failing case
    // #[case("trailing_spaces_after_key_before_colon_fail   ", "x")]
    // don't allow single backslashes, they must be part of an escape

    // #[rstest]
    // #[case("alfa:")]
    // #[case("bravo: charlie")]
    // #[case("delta: ")]
    // #[case("no_escaped_\\allowed")]
    // fn section_flag_attr_v42_invalid_flags(#[case] left: &str) {
    //     let source = format!("-- {}", left);
    //     let result = section_flag_attr_v42(source.as_str());
    //     if result.is_err() {
    //         assert!(true)
    //     } else {
    //         dbg!(left);
    //         assert!(false)
    //     }
    // }

    //
}
