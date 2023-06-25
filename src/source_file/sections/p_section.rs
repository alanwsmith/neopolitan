use crate::source_file::block::attributes;
use crate::source_file::block::block;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::bytes::complete::take_until;
use nom::character::complete::line_ending;
use nom::character::complete::multispace0;
use nom::character::complete::not_line_ending;
use nom::combinator::eof;
use nom::combinator::rest;
use nom::multi::many0;
use nom::multi::many_till;
use nom::sequence::tuple;
use nom::IResult;
use nom::Parser;

// impl SourceFile {
pub fn p_section<'a>(source: &'a str) -> IResult<&str, Option<String>> {
    let (a, r) = many0(
        tuple((multispace0, tag(">> "), not_line_ending, line_ending)).map(|(w, x, y, q)| ("", y)),
    )(source)?;

    let delta = attributes(&r.iter().map(|x| x.1).collect::<Vec<&str>>(), 0);

    let (_, b) = many_till(
        tuple((
            multispace0,
            alt((
                tuple((take_until("\n\n"), tag("\n\n"))).map(|x: (&str, &str)| x.0.trim()),
                rest.map(|x: &str| x.trim()),
            )),
        )),
        eof,
    )(a)?;
    let mut output = String::from("");
    b.0.iter().for_each(|x| {
        output.push_str("<p");
        output.push_str(&delta);
        output.push_str(">");
        output.push_str(block(x.1).unwrap().1.as_str());
        output.push_str("</p>");
    });
    Ok(("", Some(output)))
}

#[cfg(test)]

mod test {
    use crate::source_file::sections::p_section::p_section;
    use crate::source_file::SourceFile;
    use std::path::PathBuf;

    #[test]
    pub fn test_attributes_work() {
        let lines = vec![">> class: highlighted", "", "Add salt", "", ""];
        assert_eq!(
            Some(String::from(r#"<p class="highlighted">Add salt</p>"#)),
            p_section(lines.join("\n").as_str()).unwrap().1,
        )
    }

    // TODO: Update these tests to call
    // p_section directly instead of going
    // throught the SourceFile

    #[test]
    pub fn test_single_paragraph() {
        let lines = vec![
            "-> p",
            "",
            "This is a test run of the website builder",
            "",
            "",
        ];
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
        let lines = vec!["-> p", "", "Hotel India", "", "Oscar Echo", "", ""];
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
