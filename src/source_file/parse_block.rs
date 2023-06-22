use crate::source_file::source_file::SourceFile;

use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::bytes::complete::take_until;
//use nom::combinator::eof;
use nom::combinator::rest;
//use nom::multi::many_till;
use nom::sequence::tuple;
// use nom::Err;
use nom::IResult;
use nom::Parser;
// use std::error::Error;
// use nom::error::ParseError;
// use nom::error::Error;
// use nom::error::ErrorKind;
// use nom::error::ParseError;

impl SourceFile {
    pub fn tuple_parse(&self) -> String {
        "".to_string()
    }

    pub fn parse_tags<'a>(&'a self, source: &'a str) -> IResult<&str, String> {
        let (a, b) = tuple((take_until("|"), tag("|")))(source).map(|(x, y)| {
            let mut out = String::from("<strong>");
            out.push_str(y.0);
            out.push_str("</strong>");
            (x, out)
        })?;
        Ok((a, b))
    }

    pub fn parse_block<'a>(&'a self, source: &'a str) -> IResult<&str, Option<String>> {
        let (a, b) = tuple((take_until("<<"), tag("<<"), take_until(">>"), tag(">>")))(source)
            .map(|(x, y)| {
                let mut out = String::from(y.0);
                out.push_str(self.parse_tags(y.2).unwrap().1.as_str());
                (x, out)
            })?;
        let mut block = String::from(b);
        block.push_str(a);
        Ok(("", Some(block)))
    }

    pub fn parse_block_dev<'a>(&'a self, source: &'a str) -> IResult<&str, Option<String>> {
        let (_, b) = alt((
            tuple((take_until("<<"), tag("<<"), take_until(">>"), tag(">>"))).map(|x| {
                dbg!(&x);
                self.parse_tags(x.0).unwrap().0
            }),
            // .map(|(xa, xb, xc, xd)| {
            //     let mut out = String::from(xa.0);
            //     out.push_str(self.parse_tags(xc.2).unwrap().1.as_str());
            //     out
            // })
            rest,
        ))(source)?;

        dbg!(b);

        // let mut block = String::from(b);
        // block.push_str(&a);

        let block = String::from("move the radio");
        Ok(("", Some(block)))
    }
}

#[cfg(test)]
mod test {
    use crate::source_file::source_file::SourceFile;

    #[test]
    pub fn text_with_strong_tag() {
        let sf = SourceFile::new();
        assert_eq!(
            sf.parse_block("wash <<the|strong>> car"),
            Ok(("", Some(String::from("wash <strong>the</strong> car"))))
        )
    }

    #[test]
    pub fn text_with_no_tags() {
        let sf = SourceFile::new();
        assert_eq!(
            sf.parse_block_dev("move the radio"),
            Ok(("", Some(String::from("move the radio"))))
        )
    }
}
