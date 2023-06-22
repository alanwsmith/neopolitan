#![allow(warnings)]
use crate::source_file::source_file::SourceFile;
use nom::bytes::complete::tag;
// use nom::bytes::complete::take_till;
use nom::bytes::complete::take_until;
// use nom::combinator::eof;
use minijinja::context;
use minijinja::Environment;
use minijinja::Source;
use nom::branch::alt;
use nom::bytes::complete::tag_no_case;
use nom::character::complete::line_ending;
use nom::character::complete::multispace0;
use nom::character::complete::not_line_ending;
use nom::combinator::eof;
use nom::combinator::rest;
use nom::multi::many_till;
use nom::sequence::tuple;
use nom::IResult;
use nom::Parser;

impl SourceFile {
    pub fn title_section(&self, source: &str) -> Option<String> {
        let mut env = Environment::new();
        env.set_source(Source::from_path("./templates"));
        let wrapper = env.get_template("sections/title.j2").unwrap();
        Some(
            wrapper
                .render(context!(
                    title => String::from(source.trim()),
                ))
                .unwrap()
                .to_string(),
        )
    }

    pub fn title_section_dev(&self, source: &str) -> IResult<&str, Option<String>> {
        let mut env = Environment::new();
        env.set_source(Source::from_path("./templates"));
        let wrapper = env.get_template("sections/title.j2").unwrap();
        Ok((
            "",
            Some(
                wrapper
                    .render(context!(
                        title => String::from(source.trim()),
                    ))
                    .unwrap()
                    .to_string(),
            ),
        ))
    }

    pub fn p_section(&self, source: &str) -> Option<String> {
        let mut env = Environment::new();
        env.set_source(Source::from_path("./templates"));
        let wrapper = env.get_template("sections/p.j2").unwrap();
        Some(
            wrapper
                .render(context!(
                    content => String::from(source.trim()),
                ))
                .unwrap()
                .to_string(),
        )
    }

    pub fn content(&self) -> Option<String> {
        let (_, b) = self.parse_content().unwrap();
        b
    }

    pub fn p_section_dev<'a>(&'a self, source: &'a str) -> IResult<&str, Option<String>> {
        let (a, b) = many_till(
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

    fn parse_content(&self) -> IResult<&str, Option<String>> {
        let (a, b) = many_till(
            tuple((
                multispace0,
                tag_no_case("-> "),
                alt((
                    tuple((
                        tag_no_case("title"),
                        not_line_ending,
                        line_ending,
                        alt((take_until("\n\n-> "), rest)),
                    ))
                    .map(|t| self.title_section_dev(t.3)),
                    tuple((
                        tag_no_case("p"),
                        not_line_ending,
                        line_ending,
                        alt((take_until("\n\n-> "), rest)),
                    ))
                    .map(|t| self.p_section_dev(t.3)),
                    tuple((
                        tag_no_case("categories"),
                        not_line_ending,
                        line_ending,
                        alt((take_until("\n\n-> "), rest)),
                    ))
                    .map(|t| Ok(("", Some("".to_string())))),
                    tuple((
                        tag_no_case("attributes"),
                        not_line_ending,
                        line_ending,
                        alt((take_until("\n\n-> "), rest)),
                    ))
                    .map(|t| Ok(("", Some("".to_string())))),
                    // When all section types are in place this
                    // rest.map should be able to be removed. Right
                    // now it's just catching things that haven't been
                    // defined yet
                    rest.map(|x| Ok(("", Some(String::from(x))))),
                )),
            )),
            eof,
        )(self.source_data.as_ref().unwrap().as_str())?;
        let mut output = String::from("");
        b.0.iter()
            .for_each(|x| output.push_str(x.2.as_ref().unwrap().1.as_ref().unwrap().as_str()));
        Ok(("", Some(output)))
    }

    //
}

#[cfg(test)]

mod test {
    use crate::source_file::source_file::SourceFile;

    #[test]
    pub fn test_title() {
        let mut sf = SourceFile::new();
        let lines = vec!["-> title", "", "Delta Hotel"];
        sf.source_data = Some(lines.join("\n"));
        assert_eq!(
            sf.content(),
            Some(String::from(r#"<h1 class="neo-title">Delta Hotel</h1>"#))
        );
    }

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

    #[test]
    pub fn test_multiple_section() {
        let mut sf = SourceFile::new();
        let lines = vec![
            "-> title",
            "",
            "Echo Foxtrot",
            "",
            "-> p",
            "",
            "Light the candle",
        ];
        sf.source_data = Some(lines.join("\n"));
        assert_eq!(
            sf.content(),
            Some(String::from(
                r#"<h1 class="neo-title">Echo Foxtrot</h1><p>Light the candle</p>"#
            ))
        );
    }

    #[test]
    pub fn ignore_categories() {
        let mut sf = SourceFile::new();
        let lines = vec![
            "-> title",
            "",
            "Whiskey November",
            "",
            "-> categories",
            ">> Example",
        ];
        sf.source_data = Some(lines.join("\n"));
        assert_eq!(
            sf.content(),
            Some(String::from(
                r#"<h1 class="neo-title">Whiskey November</h1>"#
            ))
        );
    }

    #[test]
    pub fn ignore_attributes() {
        let mut sf = SourceFile::new();
        let lines = vec![
            "-> title",
            "",
            "Echo Oscar",
            "",
            "-> attributes",
            ">> Example",
        ];
        sf.source_data = Some(lines.join("\n"));
        assert_eq!(
            sf.content(),
            Some(String::from(r#"<h1 class="neo-title">Echo Oscar</h1>"#))
        );
    }

    #[test]
    pub fn multiple_paragraphs() {
        let mut sf = SourceFile::new();
        let lines = vec!["-> p", "", "Hotel India", "", "Oscar Echo", ""];
        sf.source_data = Some(lines.join("\n"));
        assert_eq!(
            sf.content(),
            Some(String::from(r#"<p>Hotel India</p><p>Oscar Echo</p>"#))
        );
    }

    //
}
