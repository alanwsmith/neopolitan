use crate::block_metadata::RawBlockMetaData;
use crate::block_metadata::parent::BlockParent;
use crate::config::Config;
use crate::span_metadata::parsers::span_of_plain_text_for_block_key_value_attr_value::span_of_plain_text_for_section_key_value_attr_value as span_of_plain_text_for_block_key_value_attr_value;
use crate::span_metadata::strings::space0_line_ending_or_eof::space0_line_ending_or_eof;
use nom::Parser;
use nom::character::complete::alphanumeric1;
use nom::character::complete::space1;
use nom::multi::many1;
use nom::sequence::terminated;
use nom::{IResult, branch::alt, bytes::complete::tag};

// TODO: NOTE that tags and shorthands to split
// to following lines before being closed. Show
// some examples of that.

// TODO: Figure out if you want to have the
// ability to have other span types inside
// the attrs. It would involved having to
// make new stuff to deal with newlines
// probably. But maybe it's just the text
// one that needs to deal with that and
// the criteria for usage is that you
// don't have other span types break over lines.
//
// Though, in the long term, it would be
// nice to be able to do that too.
//
// Might be able to just slurp all the
// text to start with and then parse
// it after multiple instances of the
// same attr have been assembled.
//
// TODO: Allow attrs to move to the next
// line without a preable, like
//
// -- alt: this is some alt text
// that breaks to a new line
// -- class: some classes
//

pub fn raw_block_attr<'a>(
    source: &'a str,
    _config: &'a Config,
    _parent: &'a BlockParent,
) -> IResult<&'a str, RawBlockMetaData> {
    let (source, _) = tag("--").parse(source)?;
    let (source, _) = space1.parse(source)?;

    // TODO: Figure out how this works with
    // accent characters.
    let (source, key_parts) =
        many1(alt((alphanumeric1, tag("-"), tag("_")))).parse(source)?;
    let (source, _) = tag(":").parse(source)?;
    let (source, _) = space1.parse(source)?;
    let (source, spans) = terminated(
        many1(alt((
            span_of_plain_text_for_block_key_value_attr_value,
            // link_shorthand_span,
            // footnote_shorthand_span,
            // code_span,
            // emphasis_shorthand_span,
            // html_shorthand_span,
            // image_shorthand_span,
            // mark_shorthand_span,
            // strikethrough_shorthand_span,
            // strong_shorthand_span,
            // underline_shorthand_span,
        ))),
        space0_line_ending_or_eof,
    )
    .parse(source)?;
    Ok((
        source,
        RawBlockMetaData::Attribute {
            key: key_parts.join("").to_string(),
            spans,
        },
    ))
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::span::Span;
    use pretty_assertions::assert_eq;
    // use rstest::rstest;

    #[test]
    fn block_attribute_line_basic_test() {
        let config = &Config::default();
        let parent = &BlockParent::Basic;
        let source = "-- alfa: bravo";
        let left = RawBlockMetaData::Attribute {
            key: "alfa".to_string(),
            spans: vec![Span::Text {
                content: "bravo".to_string(),
kind: "text-span".to_string()
            }],
        };
        let right = raw_block_attr(source, config, parent).unwrap().1;
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
    //     let config = &Config::default();
    //     let parent = &SectionParent::Basic;
    //     let right = section_attribute(source, config, parent).unwrap().1;
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
    //     let config = &Config::default();
    //     let parent = &SectionParent::Basic;
    //     let results = section_attribute(source, config, parent);
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
