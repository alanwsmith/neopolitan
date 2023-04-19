use nom::branch::alt;
use nom::bytes::complete::is_not;
use nom::bytes::complete::tag;
use nom::bytes::complete::take_until;
use nom::character::complete::multispace0;
use nom::character::complete::multispace1;
use nom::error::Error;
use nom::sequence::tuple;
use nom::IResult;
use nom::Parser;

#[derive(Debug, PartialEq)]
pub enum Snippet {
    Plain {
        text: Option<String>,
    },
    Kbd {
        attributes: Option<Vec<(Option<String>, Option<String>)>>,
        text: Option<String>,
    },
}

pub fn snippet(source: &str) -> IResult<&str, Snippet> {
    // This is for individaul sections of a block like
    // raw plain text, code, strong, links, etc...
    // dbg!(source);
    Ok((
        "",
        Snippet::Plain {
            text: Some(source.to_string()),
        },
    ))
}

pub fn snippet_dev(source: &str) -> IResult<&str, Snippet> {
    let (remainder, captured) = alt((
        tuple((
            multispace1::<&str, Error<&str>>,
            tag("<<"),
            is_not("|"),
            tag("|"),
            multispace0,
            tag("kbd"),
            take_until(">>"),
            tag(">>"),
        ))
        .map(|x| Snippet::Kbd {
            attributes: None,
            text: Some(x.2.to_string()),
        }),
        tuple((
            multispace1::<&str, Error<&str>>,
            multispace1,
            tag("<<"),
            is_not("|"),
            tag("|"),
            multispace0,
            tag("kbd"),
            take_until(">>"),
            tag(">>"),
        ))
        .map(|x| Snippet::Kbd {
            attributes: None,
            text: Some(x.2.to_string()),
        }),
    ))(source)?;
    // dbg!(captured);

    // This is for individaul sections of a block like
    // raw plain text, code, strong, links, etc...
    // dbg!(source);
    Ok((
        remainder,
        captured,
        // Snippet::Kbd {
        //     attributes: None,
        //     text: Some("weave the carpet".to_string()),
        // },
    ))
}

#[cfg(test)]
mod test {
    use crate::snippet::snippet::*;

    #[test]
    pub fn basic_test() {
        let expected = Snippet::Plain {
            text: Some("Take the winding path".to_string()),
        };
        let result = snippet("Take the winding path").unwrap().1;
        assert_eq!(expected, result);
    }

    #[test]
    pub fn basic_kbd_test() {
        let expected = Ok((
            " with more words",
            Snippet::Kbd {
                attributes: None,
                text: Some("weave the carpet".to_string()),
            },
        ));
        let result = snippet_dev(" <<weave the carpet|kbd>> with more words");
        assert_eq!(expected, result);
    }
}
