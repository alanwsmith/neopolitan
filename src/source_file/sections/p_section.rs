use nom::combinator::eof;
use nom::combinator::rest;
use nom::Parser;
use nom::bytes::complete::tag;
use nom::bytes::complete::take_until;
use nom::sequence::tuple;
use nom::branch::alt;
use nom::character::complete::multispace0;
use nom::multi::many_till;
use nom::IResult;
use crate::source_file::source_file::SourceFile;

impl SourceFile {
    pub fn p_section<'a>(&'a self, source: &'a str) -> IResult<&str, Option<String>> {
        let (_, b) = many_till(
            tuple((
                multispace0,
                alt((
                    tuple((take_until("\n\n"), tag("\n\n"))).map(|x: (&str, &str)| x.0.trim()),
                    rest.map(|x: &str| x.trim()),
                )),
            )),
            eof,
        )(source)?;
        let mut output = String::from("");
        b.0.iter().for_each(|x| {
            output.push_str("<p>");
            output.push_str(x.1);
            output.push_str("</p>");
            ()
        });
        Ok(("", Some(output)))
    }
}

#[cfg(test)]

mod test {
    use crate::source_file::sections::p_section::SourceFile;

    #[test]
    pub fn test_single_paragraph() {
        let mut sf = SourceFile::new();
        let lines = vec!["-> p", "", "This is a test run of the website builder"];
        sf.source_data = Some(lines.join("\n"));
        assert_eq!(
            sf.content(),
            Some(String::from(
                "<p>This is a test run of the website builder</p>"
            ))
        );
    }
}