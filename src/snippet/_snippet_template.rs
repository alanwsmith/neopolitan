$USAGE
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::bytes::complete::take_until;
use nom::character::complete::multispace0;
use nom::character::complete::multispace1;
use nom::combinator::rest;
use nom::error::Error;
use nom::sequence::tuple;
use nom::IResult;
use nom::Parser;
use crate::snippet::snippet_enum::Snippet;

pub fn snippet(source: &str) -> IResult<&str, Snippet> {
    let (remainder, captured) = alt((

        alt((
            $LINES0
        )),

        alt((
            $LINES1
        )),

        alt((
            $LINES2
        )),

        alt((
            $LINES3
        )),

        alt((
            $LINES4

            
            take_until(" <<").map(|x: &str| Snippet::Plain {
                text: Some(x.to_string()),
            }),

        rest.map(|x: &str| Snippet::Plain {
            text: Some(x.to_string()),
        }),

        )),

))(source)?;
    Ok((remainder, captured))
}
