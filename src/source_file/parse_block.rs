use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::bytes::complete::take_until;
use nom::combinator::eof;
use nom::combinator::rest;
use nom::multi::many_till;
use nom::sequence::tuple;
use nom::IResult;
use nom::Parser;

pub fn parse_block<'a>(source: &'a str) -> IResult<&str, Option<String>> {
    let (_, b) = many_till(
        alt((
            tuple((
                take_until("<<"),
                tag("<<"),
                take_until("|"),
                tag("|"),
                take_until(">>"),
                tag(">>"),
            ))
            .map(|x| {
                if x.4 == "strong" {
                    let mut out = String::from(x.0);
                    out.push_str("<strong>");
                    out.push_str(x.2);
                    out.push_str("</strong>");
                    dbg!(&out);
                    out
                } else {
                    "".to_string()
                }
            }),
            rest.map(|x: &str| x.to_string()),
        )),
        eof,
    )(source)?;
    let block = b.0.join("");
    Ok(("", Some(block)))
}


#[cfg(test)]
mod test {
    use crate::source_file::parse_block::parse_block;

    #[test]
    pub fn text_with_no_tags() {
        assert_eq!(
            parse_block("move the radio"),
            Ok(("", Some(String::from("move the radio"))))
        )
    }

    #[test]
    pub fn text_with_strong_tag() {
        assert_eq!(
            parse_block("wash <<the|strong>> car"),
            Ok(("", Some(String::from("wash <strong>the</strong> car"))))
        )
    }
}
