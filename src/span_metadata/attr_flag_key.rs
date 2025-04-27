#![allow(unused)]
use crate::span_metadata::RawSpanMetadata;
use crate::span_strings::single_character::single_character;
use nom::IResult;
use nom::Parser;
use nom::branch::alt;
use nom::bytes::complete::is_not;
use nom::bytes::complete::tag;
use nom::character::complete::line_ending;
use nom::character::complete::one_of;
use nom::character::complete::space0;
use nom::combinator::not;
use nom::combinator::opt;
use nom::combinator::peek;
use nom::combinator::recognize;
use nom::multi::many1;
use nom::sequence::preceded;
use nom::sequence::terminated;

// NOTE: Escape characters can't be used
// in attr_flag_keys.
// Using them would require switching from
// Strings to a collection of spans in
// order to rebuild the file from the AST.
// That's an unacceptable increase in
// complication.
//
// This means there's no way to use
// pipes.
//

pub fn attr_flag_key<'a>(
    source: &'a str,
    character: &'a str,
) -> IResult<&'a str, String> {
    let (source, spans) = many1(alt((
        is_not(" \t\r\n|:`~^*_<>[]{}"),
        recognize(preceded(not(tag(character)), one_of(":`~^*_<>[]{}"))),
    )))
    .parse(source)?;
    let key = spans.join("");
    Ok((source, key))
}

#[cfg(test)]
mod test {
    use super::*;
    use pretty_assertions::assert_eq;
    use rstest::rstest;

    #[rstest]
    #[case("alfa", "`", "alfa", "")]
    #[case("alfa ", "`", "alfa", " ")]
    #[case("alfa|", "`", "alfa", "|")]
    #[case("alfa~^*_<>[]{}|", "`", "alfa~^*_<>[]{}", "|")]
    #[case("alfa`^*_<>[]{}|", "~", "alfa`^*_<>[]{}", "|")]
    #[case("https://www.example.com/|", "`", "https://www.example.com/", "|")]
    fn solo_attr_flag_key_valid_tests(
        #[case] source: &str,
        #[case] character: &str,
        #[case] found: &str,
        #[case] remainder: &str,
    ) {
        let left = found.to_string();
        let right = attr_flag_key(source, character).unwrap();
        assert_eq!(left, right.1);
        assert_eq!(remainder, right.0);
    }

    // #[rstest]
    // #[case("|alfa bravo``", "`")]
    // #[case("|alfa: bravo``", "`")]
    // #[case("|alfa\\bravo``", "`")]
    // fn span_flag_invalid_tests(#[case] source: &str, #[case] character: &str) {
    //     let result = span_flag(source, character);
    //     match result {
    //         Ok(_) => {
    //             dbg!(result.unwrap());
    //             assert!(false)
    //         }
    //         Err(_) => assert!(true),
    //     }
    // }

    //
}
