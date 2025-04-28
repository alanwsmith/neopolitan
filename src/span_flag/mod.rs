use crate::span_metadata::RawSpanMetadata;
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

// NOTE: Empty spaces is trimmed from
// around the flag including newlines.

// NOTE: span_flags are straight Strings,
// not a collection of spans. Use an
// attr if you need spans

// NOTE: Spaces are not allowed in flags.
// Use an attr if you need one

// NOTE: Escaped character are not allowed
// in flags. Use an attr if you need one.
// The reason for this is if escaped
// characters were allowed, the flag
// would have to be made of spans to
// allow the source file to be reassembled
// from the AST. That's not worth the
// amount of extra effort it would
// require to work with flags in
// general

// pub fn not_character<'a>(
//     source: &'a str,
//     character: &'a str,
// ) -> IResult<&'a str, &'a str> {
//     let (source, result) =
//         recognize(preceded(not(tag(character)), one_of("`~!@#$%^&*()<>[]{}")))
//             .parse(source)?;
//     Ok((source, result))
// }

pub fn span_flag<'a>(
    source: &'a str,
    character: &'a str,
) -> IResult<&'a str, RawSpanMetadata> {
    let (source, _) =
        (tag("|"), space0, opt(line_ending), space0).parse(source)?;
    let (source, token) = span_flag_token(source, character)?;
    let (source, _) = space0.parse(source)?;
    let (source, _) = opt(line_ending).parse(source)?;
    let (source, _) = space0.parse(source)?;
    let (source, _) =
        peek(alt((tag("|"), terminated(tag(character), tag(character)))))
            .parse(source)?;
    Ok((source, RawSpanMetadata::Flag(token)))
}

fn span_flag_token<'a>(
    source: &'a str,
    character: &'a str,
) -> IResult<&'a str, String> {
    let (source, parts) = many1(alt((
        is_not(" \t\r\n\\|`~^*_>[]{}"),
        recognize(preceded(not(tag(character)), one_of("`~^*_>[]{}"))),
        span_flag_token_single_character,
    )))
    .parse(source)?;
    Ok((source, parts.join("")))
}

pub fn span_flag_token_single_character<'a>(
    source: &'a str,
) -> IResult<&'a str, &'a str> {
    let (source, token_character) = alt((
        terminated(tag("`"), peek(not(tag("`")))),
        // terminated(tag("~"), peek(not(tag("~")))),
        // terminated(tag("!"), peek(not(tag("!")))),
        // terminated(tag("@"), peek(not(tag("@")))),
        // terminated(tag("#"), peek(not(tag("#")))),
        // terminated(tag("$"), peek(not(tag("$")))),
        // terminated(tag("%"), peek(not(tag("%")))),
        // terminated(tag("^"), peek(not(tag("^")))),
        // terminated(tag("*"), peek(not(tag("*")))),
        // terminated(tag("["), peek(not(tag("[")))),
        // terminated(tag("]"), peek(not(tag("]")))),
        // terminated(tag("{"), peek(not(tag("{")))),
        // terminated(tag("}"), peek(not(tag("}")))),
        // terminated(tag("<"), peek(not(tag("<")))),
        // terminated(tag(">"), peek(not(tag(">")))),
        // terminated(tag("("), peek(not(tag("(")))),
        // terminated(tag(")"), peek(not(tag(")")))),
    ))
    .parse(source)?;
    Ok((source, token_character))
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
    #[case("alfa🕺|", "`", "alfa🕺", "|")]
    #[case("alfa`|", "`", "alfa`", "|")]
    #[case("alfa`|", "`", "alfa`", "|")]
    #[case("alfa<bravo|", ">", "alfa<bravo", "|")]
    #[case("alfa<<bravo|", ">", "alfa<<bravo", "|")]
    #[case("alfa<<<bravo|", ">", "alfa<<<bravo", "|")]
    #[case("alfa<<<<bravo|", ">", "alfa<<<<bravo", "|")]
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
        let right = span_flag_token(source, character).unwrap();
        assert_eq!(left, right.1);
        assert_eq!(remainder, right.0);
    }

    // #[rstest]
    // #[case("|alfa``", "`", "alfa", "``")]
    // #[case("|alfa ``", "`", "alfa", "``")]
    // #[case("|alfa\n``", "`", "alfa", "``")]
    // #[case("|alfa \n ``", "`", "alfa", "``")]
    // #[case("|alfa\t``", "`", "alfa", "``")]
    // #[case("|alfa\r\n``", "`", "alfa", "``")]
    // #[case("|alfa \r\n ``", "`", "alfa", "``")]
    // #[case("|\nalfa``", "`", "alfa", "``")]
    // #[case("|\talfa\t``", "`", "alfa", "``")]
    // #[case("|\r\nalfa``", "`", "alfa", "``")]
    // #[case("| \n alfa``", "`", "alfa", "``")]
    // #[case("| \r\n alfa``", "`", "alfa", "``")]
    // #[case(
    //     "|https://www.example.com/``",
    //     "`",
    //     "https://www.example.com/",
    //     "``"
    // )]
    // #[case("|alfa-bravo``", "`", "alfa-bravo", "``")]
    // #[case("|alfa_bravo``", "`", "alfa_bravo", "``")]
    // #[case("|single`character``", "`", "single`character", "``")]
    // #[case("|alfa|", "`", "alfa", "|")]
    // #[case("|alfa |", "`", "alfa", "|")]
    // #[case("|alfa\n|", "`", "alfa", "|")]
    // #[case("|alfa\t|", "`", "alfa", "|")]
    // #[case("|[[alfa]]``", "`", "[[alfa]]", "``")]
    // #[case(
    //     "|others~~!!@@##$$%%^^&&**(())<<>>[[]]{{}}``",
    //     "`",
    //     "others~~!!@@##$$%%^^&&**(())<<>>[[]]{{}}",
    //     "``"
    // )]
    // #[case(
    //     "|others``~~!!@@##$$%%^^&&**(())<<[[]]{{}}>>",
    //     ">",
    //     "others``~~!!@@##$$%%^^&&**(())<<[[]]{{}}",
    //     ">>"
    // )]
    // #[case(
    //     "|others``~~!!@@##$$%%^^&&(())<<>>[[]]{{}}**",
    //     "*",
    //     "others``~~!!@@##$$%%^^&&(())<<>>[[]]{{}}",
    //     "**"
    // )]
    // fn span_flag_valid_tests(
    //     #[case] source: &str,
    //     #[case] character: &str,
    //     #[case] found: String,
    //     #[case] remainder: &str,
    // ) {
    //     let left = RawSpanMetadata::Flag(found);
    //     let right = span_flag(source, character).unwrap();
    //     assert_eq!(left, right.1);
    //     assert_eq!(remainder, right.0);
    // }

    #[rstest]
    #[case("|alfa bravo``", "`")]
    #[case("|alfa: bravo``", "`")]
    #[case("|alfa\\bravo``", "`")]
    fn solo_span_flag_invalid_tests(
        #[case] source: &str,
        #[case] character: &str,
    ) {
        let result = span_flag(source, character);
        match result {
            Ok(_) => {
                dbg!(result.unwrap());
                assert!(false)
            }
            Err(_) => assert!(true),
        }
    }

    //
}
