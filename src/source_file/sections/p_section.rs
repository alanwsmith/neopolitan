use crate::source_file::block::block;
use crate::source_file::SourceFile;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::bytes::complete::take_until;
use nom::character::complete::multispace0;
use nom::combinator::eof;
use nom::combinator::rest;
use nom::multi::many_till;
use nom::sequence::tuple;
use nom::IResult;
use nom::Parser;

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
            output.push_str(block(x.1).unwrap().1.as_str());
            output.push_str("</p>");
        });
        Ok(("", Some(output)))
    }
}

#[cfg(test)]

mod test {
    use crate::source_file::SourceFile;
    use std::path::PathBuf;

    #[test]
    pub fn test_single_paragraph() {
        let lines = vec!["-> p", "", "This is a test run of the website builder"];
        let sf = SourceFile {
            source_data: lines.join("\n"),
            source_path: PathBuf::from(""),
        };
        assert_eq!(
            sf.content(),
            Some(String::from(
                "<p>This is a test run of the website builder</p>"
            ))
        );
    }

    #[test]
    pub fn multiple_paragraphs() {
        let lines = vec!["-> p", "", "Hotel India", "", "Oscar Echo", ""];
        let sf = SourceFile {
            source_data: lines.join("\n"),
            source_path: PathBuf::from(""),
        };
        assert_eq!(
            sf.content(),
            Some(String::from(r#"<p>Hotel India</p><p>Oscar Echo</p>"#))
        );
    }
}
