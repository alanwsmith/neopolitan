use crate::snippet::snippets::abbr::abbr;
use nom::branch::alt;
use crate::snippet::snippets::b::b;
use nom::bytes::complete::tag;
use nom::bytes::complete::take_until;
use nom::character::complete::multispace0;
use nom::character::complete::multispace1;
use nom::combinator::rest;
use nom::error::Error;
use nom::multi::separated_list0;
use nom::sequence::tuple;
use nom::IResult;
use nom::Parser;
use crate::snippet::snippet_enum::Snippet;
use crate::snippet::snippet_attribute::SnippetAttribute;

pub fn snippet(source: &str) -> IResult<&str, Snippet> {
    let (remainder, captured) = alt((
 

        
        tuple((
            multispace1::<&str, Error<&str>>,
            tag("<<"),
            take_until("|"),
            tag("|"),
            multispace0,
            tag("abbr"),
            take_until(">>"),
            tag(">>"),
        ))
        .map(|x| abbr(x.2, x.6)),


        tuple((
            multispace1::<&str, Error<&str>>,
            tag("<<"),
            take_until("|"),
            tag("|"),
            multispace0,
            tag("b"),
            take_until(">>"),
            tag(">>"),
        ))
        .map(|x| b(x.2, x.6)),


        tuple((
            multispace1::<&str, Error<&str>>,
            tag("<<"),
            take_until("|"),
            tag("|"),
            multispace0,
            tag("button"),
            take_until(">>"),
            tag(">>"),
        ))
        .map(|x| button(x.2, x.6)),


        tuple((
            multispace1::<&str, Error<&str>>,
            tag("<<"),
            take_until("|"),
            tag("|"),
            multispace0,
            tag("data"),
            take_until(">>"),
            tag(">>"),
        ))
        .map(|x| data(x.2, x.6)),


        tuple((
            multispace1::<&str, Error<&str>>,
            tag("<<"),
            take_until("|"),
            tag("|"),
            multispace0,
            tag("del"),
            take_until(">>"),
            tag(">>"),
        ))
        .map(|x| del(x.2, x.6)),


        tuple((
            multispace1::<&str, Error<&str>>,
            tag("<<"),
            take_until("|"),
            tag("|"),
            multispace0,
            tag("dfn"),
            take_until(">>"),
            tag(">>"),
        ))
        .map(|x| dfn(x.2, x.6)),


        tuple((
            multispace1::<&str, Error<&str>>,
            tag("<<"),
            take_until("|"),
            tag("|"),
            multispace0,
            tag("em"),
            take_until(">>"),
            tag(">>"),
        ))
        .map(|x| em(x.2, x.6)),


        tuple((
            multispace1::<&str, Error<&str>>,
            tag("<<"),
            take_until("|"),
            tag("|"),
            multispace0,
            tag("i"),
            take_until(">>"),
            tag(">>"),
        ))
        .map(|x| i(x.2, x.6)),


        tuple((
            multispace1::<&str, Error<&str>>,
            tag("<<"),
            take_until("|"),
            tag("|"),
            multispace0,
            tag("ins"),
            take_until(">>"),
            tag(">>"),
        ))
        .map(|x| ins(x.2, x.6)),


        tuple((
            multispace1::<&str, Error<&str>>,
            tag("<<"),
            take_until("|"),
            tag("|"),
            multispace0,
            tag("kbd"),
            take_until(">>"),
            tag(">>"),
        ))
        .map(|x| kbd(x.2, x.6)),


        tuple((
            multispace1::<&str, Error<&str>>,
            tag("<<"),
            take_until("|"),
            tag("|"),
            multispace0,
            tag("label"),
            take_until(">>"),
            tag(">>"),
        ))
        .map(|x| label(x.2, x.6)),


        tuple((
            multispace1::<&str, Error<&str>>,
            tag("<<"),
            take_until("|"),
            tag("|"),
            multispace0,
            tag("legend"),
            take_until(">>"),
            tag(">>"),
        ))
        .map(|x| legend(x.2, x.6)),


        tuple((
            multispace1::<&str, Error<&str>>,
            tag("<<"),
            take_until("|"),
            tag("|"),
            multispace0,
            tag("meter"),
            take_until(">>"),
            tag(">>"),
        ))
        .map(|x| meter(x.2, x.6)),


        tuple((
            multispace1::<&str, Error<&str>>,
            tag("<<"),
            take_until("|"),
            tag("|"),
            multispace0,
            tag("object"),
            take_until(">>"),
            tag(">>"),
        ))
        .map(|x| object(x.2, x.6)),


        tuple((
            multispace1::<&str, Error<&str>>,
            tag("<<"),
            take_until("|"),
            tag("|"),
            multispace0,
            tag("progress"),
            take_until(">>"),
            tag(">>"),
        ))
        .map(|x| progress(x.2, x.6)),


        tuple((
            multispace1::<&str, Error<&str>>,
            tag("<<"),
            take_until("|"),
            tag("|"),
            multispace0,
            tag("q"),
            take_until(">>"),
            tag(">>"),
        ))
        .map(|x| q(x.2, x.6)),


        tuple((
            multispace1::<&str, Error<&str>>,
            tag("<<"),
            take_until("|"),
            tag("|"),
            multispace0,
            tag("s"),
            take_until(">>"),
            tag(">>"),
        ))
        .map(|x| s(x.2, x.6)),


        tuple((
            multispace1::<&str, Error<&str>>,
            tag("<<"),
            take_until("|"),
            tag("|"),
            multispace0,
            tag("samp"),
            take_until(">>"),
            tag(">>"),
        ))
        .map(|x| samp(x.2, x.6)),


        tuple((
            multispace1::<&str, Error<&str>>,
            tag("<<"),
            take_until("|"),
            tag("|"),
            multispace0,
            tag("small"),
            take_until(">>"),
            tag(">>"),
        ))
        .map(|x| small(x.2, x.6)),


        tuple((
            multispace1::<&str, Error<&str>>,
            tag("<<"),
            take_until("|"),
            tag("|"),
            multispace0,
            tag("span"),
            take_until(">>"),
            tag(">>"),
        ))
        .map(|x| span(x.2, x.6)),


        tuple((
            multispace1::<&str, Error<&str>>,
            tag("<<"),
            take_until("|"),
            tag("|"),
            multispace0,
            tag("strong"),
            take_until(">>"),
            tag(">>"),
        ))
        .map(|x| strong(x.2, x.6)),


        tuple((
            multispace1::<&str, Error<&str>>,
            tag("<<"),
            take_until("|"),
            tag("|"),
            multispace0,
            tag("sub"),
            take_until(">>"),
            tag(">>"),
        ))
        .map(|x| sub(x.2, x.6)),


        tuple((
            multispace1::<&str, Error<&str>>,
            tag("<<"),
            take_until("|"),
            tag("|"),
            multispace0,
            tag("sup"),
            take_until(">>"),
            tag(">>"),
        ))
        .map(|x| sup(x.2, x.6)),


        tuple((
            multispace1::<&str, Error<&str>>,
            tag("<<"),
            take_until("|"),
            tag("|"),
            multispace0,
            tag("time"),
            take_until(">>"),
            tag(">>"),
        ))
        .map(|x| time(x.2, x.6)),


        tuple((
            multispace1::<&str, Error<&str>>,
            tag("<<"),
            take_until("|"),
            tag("|"),
            multispace0,
            tag("u"),
            take_until(">>"),
            tag(">>"),
        ))
        .map(|x| u(x.2, x.6)),


        tuple((
            multispace1::<&str, Error<&str>>,
            tag("<<"),
            take_until("|"),
            tag("|"),
            multispace0,
            tag("var"),
            take_until(">>"),
            tag(">>"),
        ))
        .map(|x| var(x.2, x.6)),

        take_until(" <<").map(|x: &str| Snippet::Plain {
            text: Some(x.to_string()),
        }),

        rest.map(|x: &str| Snippet::Plain {
            text: Some(x.to_string()),
        }),
    ))(source)?;
    Ok((remainder, captured))
}
