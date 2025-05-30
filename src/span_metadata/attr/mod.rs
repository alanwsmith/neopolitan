use crate::span::shorthand::shorthand_span;
use crate::span::text::in_span_attr::text_span_in_span_attr;
use crate::span_metadata::RawSpanMetadata;
use nom::IResult;
use nom::Parser;
use nom::branch::alt;
use nom::bytes::complete::is_a;
use nom::bytes::complete::is_not;
use nom::bytes::complete::tag;
use nom::character::complete::line_ending;
use nom::character::complete::space0;
use nom::combinator::not;
use nom::combinator::opt;
use nom::combinator::peek;
use nom::multi::many1;
use nom::sequence::terminated;

// TODO: Probably don't move everything
// into a single attr? that way
// folks can make the decision in the
// output if there should be a space
// added between the lines or not.
// Or, they could use a different
// character and use the different
// lines as an explicit separator.

// TODO: Use single function for
// attr key and flags.

// NOTE: span attrs can only have
// text spans in them for the
// first version. might look
// at adding the ability to
// nest different span types,
// but the feels like a lot
// of complexity for now a lot
// of benefit.

// NOTE: you can have the double close
// tokens for alternate tags in the
// keys. For example ``alfa**bravo: charlie``
// is valid with a key of ``alfa**bravo``
// it's possible to make any key because you
// can use either the shorthand or the
// explicit ``<<>>`` tag if you need the
// key to contain what would otherwise
// be the close tag for the shorthand

// TODO: Don't try to nest sections with the
// same tokens. Does that need a test or
// will that just break with sections
// and throw an accurate error there?

pub fn span_attr<'a>(
    source: &'a str,
    character: &'a str,
) -> IResult<&'a str, RawSpanMetadata> {
    let (source, _) =
        (tag("|"), space0, opt(line_ending), space0).parse(source)?;
    let (source, key) =
        (|src| span_attr_key_token(src, character)).parse(source)?;
    let (source, spans) =
        many1(alt((text_span_in_span_attr, shorthand_span))).parse(source)?;
    Ok((source, RawSpanMetadata::Attr { key, spans }))
}

pub fn span_attr_key_token<'a>(
    source: &'a str,
    character: &'a str,
) -> IResult<&'a str, String> {
    let (source, key_snippets) = many1(alt((
        is_not(": \t\r\n\\|~`@^*_>]})"),
        span_attr_key_token_single_character,
        |src| span_attr_key_token_alternate_close_characters(src, character),
    )))
    .parse(source)?;
    let (source, _) = tag(":").parse(source)?;
    let (source, _) = (space0, opt(line_ending), space0).parse(source)?;
    let (source, _) = peek(not(tag("|"))).parse(source)?;
    let (source, _) = not(line_ending).parse(source)?;
    let (source, _) = not((tag(character), tag(character))).parse(source)?;
    Ok((source, key_snippets.join("").to_string()))
}

pub fn span_attr_key_token_single_character(
    source: &str,
) -> IResult<&str, &str> {
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

pub fn span_attr_key_token_alternate_close_characters<'a>(
    source: &'a str,
    character: &'a str,
) -> IResult<&'a str, &'a str> {
    let (source, _) =
        peek(not((tag(character), tag(character)))).parse(source)?;
    let (source, characters) = alt((is_a("~`@^*_])}>"),)).parse(source)?;
    Ok((source, characters))
}

#[cfg(test)]
mod test {
    use std::collections::BTreeMap;

    use super::*;
    use crate::span::Span;
    use pretty_assertions::assert_eq;
    use rstest::rstest;

    // Original tests to review below this line

    // NOTE: These rstest lines are long because
    // breaking them breaks the formatter
    // for the entire file. New tests that need
    // to be long are made in independent
    // tests below.

    // TODO: Add invalid test where there is multiple
    // lines between the `|` and different parts
    // of the attr
    //

    #[rstest]
    #[case("alfa: bravo``", "`", "alfa", "bravo``")]
    #[case("alfa: bravo|", "`", "alfa", "bravo|")]
    #[case("alfa:    bravo``", "`", "alfa", "bravo``")]
    #[case("alfa:\nbravo``", "`", "alfa", "bravo``")]
    #[case("alfa:\n bravo``", "`", "alfa", "bravo``")]
    #[case("alfa: \n bravo``", "`", "alfa", "bravo``")]
    #[case("alfa🕺: bravo``", "`", "alfa🕺", "bravo``")]
    #[case("alfa`bravo: charlie", "`", "alfa`bravo", "charlie")]
    fn span_attr_key_token_valid_tests(
        #[case] source: &str,
        #[case] character: &str,
        #[case] left: String,
        #[case] remainder: &str,
    ) {
        let right = span_attr_key_token(source, character).unwrap();
        assert_eq!(left, right.1);
        assert_eq!(remainder, right.0);
    }

    #[rstest]
    #[case("alfa~: bravo", "~", "alfa~", "bravo")]
    #[case("alfa~bravo: charlie", "~", "alfa~bravo", "charlie")]
    #[case("alfa~~bravo: charlie", "`", "alfa~~bravo", "charlie")]
    #[case("alfa`: bravo", "`", "alfa`", "bravo")]
    #[case("alfa`bravo: charlie", "`", "alfa`bravo", "charlie")]
    #[case("alfa``bravo: charlie", "*", "alfa``bravo", "charlie")]
    #[case("alfa@: bravo", "@", "alfa@", "bravo")]
    #[case("alfa@bravo: charlie", "@", "alfa@bravo", "charlie")]
    #[case("alfa@@bravo: charlie", "`", "alfa@@bravo", "charlie")]
    #[case("alfa^: bravo", "^", "alfa^", "bravo")]
    #[case("alfa^bravo: charlie", "^", "alfa^bravo", "charlie")]
    #[case("alfa^^bravo: charlie", "`", "alfa^^bravo", "charlie")]
    #[case("alfa*: bravo", "*", "alfa*", "bravo")]
    #[case("alfa*bravo: charlie", "*", "alfa*bravo", "charlie")]
    #[case("alfa**bravo: charlie", "`", "alfa**bravo", "charlie")]
    #[case("alfa_: bravo", "_", "alfa_", "bravo")]
    #[case("alfa_bravo: charlie", "_", "alfa_bravo", "charlie")]
    #[case("alfa__bravo: charlie", "`", "alfa__bravo", "charlie")]
    #[case("alfa>: bravo", ">", "alfa>", "bravo")]
    #[case("alfa>bravo: charlie", ">", "alfa>bravo", "charlie")]
    #[case("alfa>>bravo: charlie", "`", "alfa>>bravo", "charlie")]
    #[case("alfa]: bravo", "]", "alfa]", "bravo")]
    #[case("alfa]bravo: charlie", "]", "alfa]bravo", "charlie")]
    #[case("alfa]]bravo: charlie", "`", "alfa]]bravo", "charlie")]
    #[case("alfa}: bravo", "}", "alfa}", "bravo")]
    #[case("alfa}bravo: charlie", "}", "alfa}bravo", "charlie")]
    #[case("alfa}}bravo: charlie", "`", "alfa}}bravo", "charlie")]
    #[case("alfa): bravo", ")", "alfa)", "bravo")]
    #[case("alfa)bravo: charlie", ")", "alfa)bravo", "charlie")]
    #[case("alfa))bravo: charlie", "`", "alfa))bravo", "charlie")]
    fn span_attr_key_token_generated_valid_tests(
        #[case] source: &str,
        #[case] character: &str,
        #[case] left: String,
        #[case] remainder: &str,
    ) {
        let right = span_attr_key_token(source, character).unwrap();
        assert_eq!(left, right.1);
        assert_eq!(remainder, right.0);
    }

    #[rstest]
    #[case("alfa:   ``", "`")]
    #[case("alfa:\n\nbravo``", "`")]
    #[case("alfa:|bravo``", "`")]
    #[case("alfa: |bravo``", "`")]
    #[case("alfa:\n|bravo``", "`")]
    #[case("alfa bravo: charlie``", "`")]
    #[case("alfa``bravo: charlie", "`")]
    #[case("alfa**x", "*")]
    #[case("alfa__x", "_")]
    #[case("alfa^^x", "^")]
    #[case("alfa@@x", "@")]
    #[case("alfa~~x", "~")]
    #[case("alfa))x", ")")]
    #[case("alfa]]x", "]")]
    #[case("alfa}}x", "}")]
    #[case("alfa>>x", ">")]
    fn span_attr_key_token_invalid_tests(
        #[case] source: &str,
        #[case] character: &str,
    ) {
        let result = span_attr_key_token(source, character);
        match result {
            Ok(_) => {
                dbg!(result.unwrap());
                assert!(false)
            }
            Err(_) => assert!(true),
        }
    }

    #[rstest]
    #[case("|alfa: bravo``", "`", RawSpanMetadata::Attr{ key: "alfa".to_string(), spans: vec![Span::Text{content: "bravo".to_string(), kind: "text".to_string()}]} , "``")]
    #[case("| alfa: bravo``", "`", RawSpanMetadata::Attr{ key: "alfa".to_string(), spans: vec![Span::Text{content: "bravo".to_string(), kind: "text".to_string()}]} , "``")]
    #[case("| \n alfa: bravo``", "`", RawSpanMetadata::Attr{ key: "alfa".to_string(), spans: vec![Span::Text{content: "bravo".to_string(), kind: "text".to_string()}]} , "``")]
    #[case("|alfa:\nbravo``", "`", RawSpanMetadata::Attr{ key: "alfa".to_string(), spans: vec![Span::Text{content: "bravo".to_string(), kind: "text".to_string()}]} , "``")]
    #[case("|alfa:\n bravo``", "`", RawSpanMetadata::Attr{ key: "alfa".to_string(), spans: vec![Span::Text{content: "bravo".to_string(), kind: "text".to_string()}]} , "``")]
    #[case("|alfa!!@@##$$%%^^&&**(())[[]]{{}}<<>>:\n bravo``", "`", RawSpanMetadata::Attr{ key: "alfa!!@@##$$%%^^&&**(())[[]]{{}}<<>>".to_string(), spans: vec![Span::Text{content: "bravo".to_string(), kind: "text".to_string()}]} , "``")]
    fn span_attr_valid_tests(
        #[case] source: &str,
        #[case] character: &str,
        #[case] left: RawSpanMetadata,
        #[case] remainder: &str,
    ) {
        let right = span_attr(source, character).unwrap();
        assert_eq!(left, right.1);
        assert_eq!(remainder, right.0);
    }

    #[test]
    fn span_attr_whitespace_test() {
        let source = "|  delta: \n   sierra  \n yankee  \n  `` ping";
        let character = "`";
        let left = RawSpanMetadata::Attr {
            key: "delta".to_string(),
            spans: vec![Span::Text {
                content: "sierra yankee ".to_string(),
                kind: "text".to_string(),
            }],
        };
        let remainder = "`` ping";
        let right = span_attr(source, character).unwrap();
        assert_eq!(left, right.1);
        assert_eq!(remainder, right.0);
    }

    #[test]
    fn span_attr_string_closing_test() {
        let source = "|alfa: ``bravo``>> charlie";
        let character = ">";
        let left = RawSpanMetadata::Attr {
            key: "alfa".to_string(),
            spans: vec![Span::Code {
                attrs: BTreeMap::new(),
                flags: vec![],
                kind: "code-shorthand".to_string(),
                spans: vec![Span::Text {
                    content: "bravo".to_string(),
                    kind: "text".to_string(),
                }],
            }],
        };
        let remainder = ">> charlie";
        let right = span_attr(source, character).unwrap();
        assert_eq!(left, right.1);
        assert_eq!(remainder, right.0);
    }

    //
}
