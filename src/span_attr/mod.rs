use crate::span::code_span::code_span;
use crate::span::text_span::text_span;
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
use nom::combinator::recognize;
use nom::multi::many1;
use nom::sequence::preceded;

// TODO: Verify the same rules
// for flags apply for attribute
// keys

pub fn not_character<'a>(
    source: &'a str,
    character: &'a str,
) -> IResult<&'a str, &'a str> {
    let (source, result) =
        recognize(preceded(not(tag(character)), one_of("`~!@#$%^&*()<>[]{}")))
            .parse(source)?;
    Ok((source, result))
}

pub fn span_attr<'a>(
    source: &'a str,
    character: &'a str,
) -> IResult<&'a str, RawSpanMetadata> {
    let (source, _) =
        (tag("|"), space0, opt(line_ending), space0).parse(source)?;
    let (source, key_snippets) = many1(alt((
        is_not(": \r\n\t\\|~`!@#$%^&*()<>[]{}"),
        single_character,
        |src| not_character(src, character),
    )))
    .parse(source)?;
    let (source, _) = tag(":").parse(source)?;
    let (source, _) = (space0, opt(line_ending), space0).parse(source)?;
    let (source, spans) = many1(alt((text_span, code_span))).parse(source)?;
    Ok((
        source,
        RawSpanMetadata::Attr {
            key: key_snippets.join("").to_string(),
            spans,
        },
    ))
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::span::Span;
    use pretty_assertions::assert_eq;
    use rstest::rstest;
    use std::collections::BTreeMap;

    // NOTE: These lines are long because
    // breaking them breaks the formatter
    // for the entire file

    #[rstest]
    #[case("|alfa: bravo``", "`", RawSpanMetadata::Attr{ key: "alfa".to_string(), spans: vec![Span::Text{content: "bravo".to_string()}]} , "``")]
    #[case("| alfa: bravo``", "`", RawSpanMetadata::Attr{ key: "alfa".to_string(), spans: vec![Span::Text{content: "bravo".to_string()}]} , "``")]
    #[case("| \n alfa: bravo``", "`", RawSpanMetadata::Attr{ key: "alfa".to_string(), spans: vec![Span::Text{content: "bravo".to_string()}]} , "``")]
    #[case("|alfa:\nbravo``", "`", RawSpanMetadata::Attr{ key: "alfa".to_string(), spans: vec![Span::Text{content: "bravo".to_string()}]} , "``")]
    #[case("|alfa:\n bravo``", "`", RawSpanMetadata::Attr{ key: "alfa".to_string(), spans: vec![Span::Text{content: "bravo".to_string()}]} , "``")]
    #[case("|alfa!!@@##$$%%^^&&**(())[[]]{{}}<<>>:\n bravo``", "`", RawSpanMetadata::Attr{ key: "alfa!!@@##$$%%^^&&**(())[[]]{{}}<<>>".to_string(), spans: vec![Span::Text{content: "bravo".to_string()}]} , "``")]
    #[case("|alfa: bravo``code-here``>>", ">",RawSpanMetadata::Attr{ key: "alfa".to_string(), spans: vec![Span::Text{content: "bravo".to_string()}, Span::Code{attrs: BTreeMap::new(), flags: vec![], spans: vec![Span::Text{content: "code-here".to_string()}]}]}, ">>")]
    fn span_flag_valid_tests(
        #[case] source: &str,
        #[case] character: &str,
        #[case] left: RawSpanMetadata,
        #[case] remainder: &str,
    ) {
        let right = span_attr(source, character).unwrap();
        assert_eq!(left, right.1);
        assert_eq!(remainder, right.0);
    }
}
