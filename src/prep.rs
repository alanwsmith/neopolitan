use crate::rawblock::RawBlock;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::bytes::complete::take_until;
use nom::character::complete::line_ending;
use nom::character::complete::multispace0;
use nom::character::complete::space0;
use nom::combinator::eof;
use nom::combinator::rest;
use nom::multi::many_till;
use nom::sequence::tuple;
use nom::IResult;

/////////////////////////////////////////////
// Note: this only requires one line to be above
// the `-> ` token. Or, more to the point it just
// looks for a newline before it. Ideally, it would
// be two, but that's more complicated than I want
// to put the time into for now.
//
// Two empty lines are required after a header.
// They can have whitespace but no other characters.
/////////////////////////////////////////////

// #[derive(Debug, PartialEq)]
// enum RawBlock {
//     Title { text: String },
//     P { text: String },
//     Code { text: String },
//     Error { text: String },
// }

pub fn prep(source: &str) -> IResult<&str, Vec<RawBlock>> {
    let (_, tokens) = many_till(do_split, eof)(source)?;
    Ok(("", tokens.0))
}

fn newline_with_whitespace(source: &str) -> IResult<&str, &str> {
    let (source, _) = space0(source)?;
    let (source, value) = line_ending(source)?;
    Ok((source, value))
}

fn do_split(source: &str) -> IResult<&str, RawBlock> {
    let (source, _) = multispace0(source)?;
    let (source, value) = tuple((
        alt((
            tag("-> ATTRIBUTES"),
            tag("-> BLURB"),
            tag("-> CATEGORIES"),
            tag("-> CODE"),
            tag("-> DIV"),
            tag("-> H1"),
            tag("-> H2"),
            tag("-> H3"),
            tag("-> H4"),
            tag("-> H5"),
            tag("-> H6"),
            tag("-> P"),
            tag("-> TITLE"),
        )),
        newline_with_whitespace,
        alt((take_until("\n-> "), rest)),
    ))(source)?;
    let string_value = value.2.trim().to_string();
    let response = match value.0 {
        "-> ATTRIBUTES" => RawBlock::Attributes { text: string_value },
        "-> BLURB" => RawBlock::Blurb { text: string_value },
        "-> CATEGORIES" => RawBlock::Categories { text: string_value },
        "-> CODE" => RawBlock::Code { text: string_value },
        "-> DIV" => RawBlock::Div { text: string_value },
        "-> H2" => RawBlock::H2 { text: string_value },
        "-> TITLE" => RawBlock::Title { text: string_value },
        "-> P" => RawBlock::P { text: string_value },
        _ => RawBlock::Error { text: string_value },
    };
    Ok((source, response))
}
