use crate::span::metadata::RawSpanMetadata;
use nom::IResult;
use nom::Parser;
use nom::branch::alt;
use nom::bytes::complete::is_not;
use nom::bytes::complete::tag;
use nom::character::complete::line_ending;
use nom::character::complete::one_of;
use nom::character::complete::space0;
use nom::character::complete::space1;
use nom::combinator::not;
use nom::combinator::opt;
use nom::combinator::peek;
use nom::combinator::recognize;
use nom::multi::many1;
use nom::sequence::preceded;
use nom::sequence::terminated;

// NOTE: This makes the assumption that
// attrs have already been identified
// and pulled out so they aren't
// passed in to these functions. I expect
// there's a way to build this so
// it wouldn't pick up attrs regardless,
// but the order will ensure that's not
// necessary. Just want to make sure
// there's a note of that. What it
// amounts to is that flags will
// pick up anything that's not
// an attr.

// TODO: Allow colons in the text.
// The criteria is that if it's
// on the first word it's an attr.
// otherwise, they can be in there.
// (so, anything after the first space
// is allowed) This is to allow for
// strings of text that you so
// they don't break things, for example
// if you make a comment after a
// tlink to know what it goes to
//

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

// Using the tokens from another tag/shorthand
// is allowed. For example, this
//
// ``ping|ping<<>>``
//
// produces a valid flag of: ping<<>>
//
// where as this:
//
// <<ping|ping<<>>>>
//
// produces a flag of "ping<<" with a remainder
// of ">>" in the parser.
//
// This introduces some inconsistent behavior
// in that any given token set can be used
// in everything except itself. I'm okay with
// this given the trade off is expanding the
// overall possibilities without introducing
// escape characters.
//

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
        is_not(" \t\r\n\\|~`@^*_>]})"),
        recognize(preceded(not(tag(character)), one_of("~`@^*_>]})"))),
        (space0, line_ending, space0, not(line_ending)).map(|_| " "),
        (space1, line_ending, space0, not(line_ending)).map(|_| " "),
        space1,
        span_flag_token_single_character,
    )))
    .parse(source)?;
    Ok((source, parts.join("").trim().to_string()))
}

pub fn span_flag_token_single_character<'a>(
    source: &'a str,
) -> IResult<&'a str, &'a str> {
    let (source, token_character) = alt((
        terminated(tag("~"), peek(not(tag("~")))),
        terminated(tag("`"), peek(not(tag("`")))),
        terminated(tag("@"), peek(not(tag("@")))),
        terminated(tag("^"), peek(not(tag("^")))),
        terminated(tag("*"), peek(not(tag("*")))),
        terminated(tag("_"), peek(not(tag("_")))),
        terminated(tag("]"), peek(not(tag("]")))),
        terminated(tag(")"), peek(not(tag(")")))),
        terminated(tag("}"), peek(not(tag("}")))),
        terminated(tag(">"), peek(not(tag(">")))),
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
    #[case("|alfa``", "`", "alfa", "``")]
    #[case("|alfa ``", "`", "alfa", "``")]
    #[case("|alfa\n``", "`", "alfa", "``")]
    #[case("|alfa \n ``", "`", "alfa", "``")]
    #[case("|alfa\t``", "`", "alfa", "``")]
    #[case("|alfa\r\n``", "`", "alfa", "``")]
    #[case("|alfa \r\n ``", "`", "alfa", "``")]
    #[case("|\nalfa``", "`", "alfa", "``")]
    #[case("|\talfa\t``", "`", "alfa", "``")]
    #[case("|\r\nalfa``", "`", "alfa", "``")]
    #[case("| \n alfa``", "`", "alfa", "``")]
    #[case("| \r\n alfa``", "`", "alfa", "``")]
    #[case("|alfa-bravo``", "`", "alfa-bravo", "``")]
    #[case("|alfa_bravo``", "`", "alfa_bravo", "``")]
    #[case("|single`character``", "`", "single`character", "``")]
    #[case("|alfa|", "`", "alfa", "|")]
    #[case("|alfa |", "`", "alfa", "|")]
    #[case("|alfa\n|", "`", "alfa", "|")]
    #[case("|alfa\t|", "`", "alfa", "|")]
    #[case("|[[alfa]]``", "`", "[[alfa]]", "``")]
    #[case("|alfa bravo``", "`", "alfa bravo", "``")]
    #[case("|alfa bravo: charlie``", "`", "alfa bravo: charlie", "``")]
    fn span_flag_valid_tests(
        #[case] source: &str,
        #[case] character: &str,
        #[case] found: String,
        #[case] remainder: &str,
    ) {
        let left = RawSpanMetadata::Flag(found);
        let right = span_flag(source, character).unwrap();
        assert_eq!(left, right.1);
        assert_eq!(remainder, right.0);
    }

    #[rstest]
    #[case("alfa", "`", "alfa", "")]
    #[case("alfa ", "`", "alfa", "")]
    #[case("alfa\n", "`", "alfa", "")]
    #[case("alfa|", "`", "alfa", "|")]
    #[case("alfa🕺|", "`", "alfa🕺", "|")]
    #[case("alfa` ", "`", "alfa`", "")]
    #[case("alfa`|", "`", "alfa`", "|")]
    #[case("alfa`x ", "`", "alfa`x", "")]
    #[case("alfa`x|", "`", "alfa`x", "|")]
    #[case("alfa``x", "`", "alfa", "``x")]
    #[case("alfa``|", "_", "alfa``", "|")]
    #[case("alfa* ", "*", "alfa*", "")]
    #[case("alfa*|", "*", "alfa*", "|")]
    #[case("alfa* |", "*", "alfa*", "|")]
    #[case("alfa*x ", "*", "alfa*x", "")]
    #[case("alfa*x|", "*", "alfa*x", "|")]
    #[case("alfa**x", "*", "alfa", "**x")]
    #[case("alfa**|", "_", "alfa**", "|")]
    #[case("alfa_ ", "_", "alfa_", "")]
    #[case("alfa_|", "_", "alfa_", "|")]
    #[case("alfa_ |", "_", "alfa_", "|")]
    #[case("alfa_x ", "_", "alfa_x", "")]
    #[case("alfa_x|", "_", "alfa_x", "|")]
    #[case("alfa__x", "_", "alfa", "__x")]
    #[case("alfa__|", "`", "alfa__", "|")]
    #[case("alfa^ ", "^", "alfa^", "")]
    #[case("alfa^|", "^", "alfa^", "|")]
    #[case("alfa^ |", "^", "alfa^", "|")]
    #[case("alfa^x ", "^", "alfa^x", "")]
    #[case("alfa^x|", "^", "alfa^x", "|")]
    #[case("alfa^^x", "^", "alfa", "^^x")]
    #[case("alfa^^|", "_", "alfa^^", "|")]
    #[case("alfa@ ", "@", "alfa@", "")]
    #[case("alfa@|", "@", "alfa@", "|")]
    #[case("alfa@ |", "@", "alfa@", "|")]
    #[case("alfa@x ", "@", "alfa@x", "")]
    #[case("alfa@x|", "@", "alfa@x", "|")]
    #[case("alfa@@x", "@", "alfa", "@@x")]
    #[case("alfa@@|", "_", "alfa@@", "|")]
    #[case("alfa~ ", "~", "alfa~", "")]
    #[case("alfa~|", "~", "alfa~", "|")]
    #[case("alfa~ |", "~", "alfa~", "|")]
    #[case("alfa~x ", "~", "alfa~x", "")]
    #[case("alfa~x|", "~", "alfa~x", "|")]
    #[case("alfa~~x", "~", "alfa", "~~x")]
    #[case("alfa~~|", "_", "alfa~~", "|")]
    #[case("alfa) ", ")", "alfa)", "")]
    #[case("alfa)|", ")", "alfa)", "|")]
    #[case("alfa) |", ")", "alfa)", "|")]
    #[case("alfa)x ", ")", "alfa)x", "")]
    #[case("alfa)x|", ")", "alfa)x", "|")]
    #[case("alfa))x", ")", "alfa", "))x")]
    #[case("alfa))|", "_", "alfa))", "|")]
    #[case("alfa] ", "]", "alfa]", "")]
    #[case("alfa]|", "]", "alfa]", "|")]
    #[case("alfa] |", "]", "alfa]", "|")]
    #[case("alfa]x ", "]", "alfa]x", "")]
    #[case("alfa]x|", "]", "alfa]x", "|")]
    #[case("alfa]]x", "]", "alfa", "]]x")]
    #[case("alfa]]|", "_", "alfa]]", "|")]
    #[case("alfa} ", "}", "alfa}", "")]
    #[case("alfa}|", "}", "alfa}", "|")]
    #[case("alfa} |", "}", "alfa}", "|")]
    #[case("alfa}x ", "}", "alfa}x", "")]
    #[case("alfa}x|", "}", "alfa}x", "|")]
    #[case("alfa}}x", "}", "alfa", "}}x")]
    #[case("alfa}}|", "_", "alfa}}", "|")]
    #[case("alfa> ", ">", "alfa>", "")]
    #[case("alfa>|", ">", "alfa>", "|")]
    #[case("alfa> |", ">", "alfa>", "|")]
    #[case("alfa>x ", ">", "alfa>x", "")]
    #[case("alfa>x|", ">", "alfa>x", "|")]
    #[case("alfa>>x", ">", "alfa", ">>x")]
    #[case("alfa>>|", "_", "alfa>>", "|")]
    #[case("alfa<bravo|", ">", "alfa<bravo", "|")]
    #[case("alfa<<bravo|", ">", "alfa<<bravo", "|")]
    #[case("alfa<<<bravo|", ">", "alfa<<<bravo", "|")]
    #[case("alfa<<<<bravo|", ">", "alfa<<<<bravo", "|")]
    #[case("alfa[bravo|", "]", "alfa[bravo", "|")]
    #[case("alfa[[bravo|", "]", "alfa[[bravo", "|")]
    #[case("alfa[[[bravo|", "]", "alfa[[[bravo", "|")]
    #[case("alfa[[[[bravo|", "]", "alfa[[[[bravo", "|")]
    #[case("alfa(bravo|", ">", "alfa(bravo", "|")]
    #[case("alfa((bravo|", ">", "alfa((bravo", "|")]
    #[case("alfa(((bravo|", ">", "alfa(((bravo", "|")]
    #[case("alfa((((bravo|", ">", "alfa((((bravo", "|")]
    #[case("alfa{bravo|", ">", "alfa{bravo", "|")]
    #[case("alfa{{bravo|", ">", "alfa{{bravo", "|")]
    #[case("alfa{{{bravo|", ">", "alfa{{{bravo", "|")]
    #[case("alfa{{{{bravo|", ">", "alfa{{{{bravo", "|")]
    #[case("alfa:bravo|", ">", "alfa:bravo", "|")]
    #[case("alfa::bravo|", ">", "alfa::bravo", "|")]
    #[case("alfa:::bravo|", ">", "alfa:::bravo", "|")]
    #[case("alfa::::bravo|", ">", "alfa::::bravo", "|")]
    #[case("https://www.example.com/|", "`", "https://www.example.com/", "|")]
    #[case("https://www.example.com/``", "`", "https://www.example.com/", "``")]
    #[case("alfa bravo|", ">", "alfa bravo", "|")]
    #[case("alfa\nbravo|", ">", "alfa bravo", "|")]
    #[case("alfa \n bravo|", ">", "alfa bravo", "|")]
    #[case("alfa bravo: charlie|", ">", "alfa bravo: charlie", "|")]
    #[case(":alfa bravo: charlie:|", ">", ":alfa bravo: charlie:", "|")]
    #[case("\n:alfa bravo: charlie:\n|", ">", ":alfa bravo: charlie:", "|")]
    fn span_flag_token_valid_tests(
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

    #[rstest]
    #[case("|alfa\\bravo``", "`")]
    #[case("|alfa\n\nbravo``", "`")]
    #[case("|alfa \n \n bravo``", "`")]
    fn span_flag_invalid_tests(#[case] source: &str, #[case] character: &str) {
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
