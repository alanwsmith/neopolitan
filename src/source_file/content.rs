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
        Some("<p>This is a test run of the website builder</p>".to_string())
        // self.content_dev()
    }

    pub fn content_dev(&self) -> Option<String> {
        let (_, b) = self.parse_content_dev().unwrap();
        b
    }

    fn parse_content_dev(&self) -> IResult<&str, Option<String>> {
        let (a, b) = many_till(
            tuple((
                multispace0,
                tag_no_case("-> "),
                alt((
                    tuple((
                        tag_no_case("title"),
                        not_line_ending,
                        line_ending,
                        alt((take_until("-> "), rest)),
                    ))
                    .map(|t| self.title_section(t.3)),
                    tuple((
                        tag_no_case("p"),
                        not_line_ending,
                        line_ending,
                        alt((take_until("-> "), rest)),
                    ))
                    .map(|t| self.p_section(t.3)),
                )),
            )),
            eof,
        )(self.source_data.as_ref().unwrap().as_str())?;
        let mut output = String::from("");
        b.0.iter()
            .for_each(|x| output.push_str(x.2.as_ref().unwrap().as_str()));
        Ok(("", Some(output)))
    }
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
            sf.content_dev(),
            Some(String::from(r#"<h1 class="neo-title">Delta Hotel</h1>"#))
        );
    }

    #[test]
    pub fn test_single_paragraph() {
        let mut sf = SourceFile::new();
        let lines = vec!["-> p", "", "This is a test run of the website builder"];
        sf.source_data = Some(lines.join("\n"));
        assert_eq!(
            sf.content_dev(),
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
            sf.content_dev(),
            Some(String::from(
                r#"<h1 class="neo-title">Echo Foxtrot</h1><p>Light the candle</p>"#
            ))
        );
    }

    //
}
