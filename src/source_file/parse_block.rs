#![allow(warnings)]
use crate::source_file::source_file::SourceFile;

use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::bytes::complete::take_until;
use nom::combinator::eof;
use nom::combinator::rest;
use nom::multi::many_till;
use nom::sequence::tuple;
use nom::IResult;
use nom::Parser;

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

    pub fn parse_block_old<'a>(&'a self, source: &'a str) -> IResult<&str, Option<String>> {
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

    pub fn parse_block<'a>(&'a self, source: &'a str) -> IResult<&str, Option<String>> {
        let (a, b) = many_till(
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

                    // dbg!(&x.4);
                    // let out = String::from("");
                    // dbg!(&out);
                    // out
                }),
                rest.map(|x: &str| x.to_string()),
            )),
            eof,
        )(source)?;

        dbg!(&a);
        dbg!(&b);

        let block = b.0.join("");
        dbg!(&block);

        // tuple((take_until("<<"), tag("<<"), take_until(">>"), tag(">>"))).map(|x| {
        //     dbg!(&x);
        //     let mut out = String::from(x.0);
        //     out.push_str(self.parse_tags(x.2).unwrap().1.as_str());
        //     // // // // // // // // dbg!(out);
        //     "asdf"
        // }),

        // .map(|(xa, xb, xc, xd)| {
        //     let mut out = String::from(xa.0);
        //     out.push_str(self.parse_tags(xc.2).unwrap().1.as_str());
        //     out
        // })

        // dbg!(b);

        // let mut block = String::from(b);
        // block.push_str(&a);

        // let block = String::from("move the radio");
        Ok(("", Some(block)))

        // let mut block = String::from(b);
        // block.push_str(a);
        // Ok(("", Some(block)))

        //
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
            sf.parse_block("move the radio"),
            Ok(("", Some(String::from("move the radio"))))
        )
    }
}
