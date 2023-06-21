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
use nom::character::complete::not_line_ending;
use nom::combinator::rest;
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
        //

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

        //

        // let mut output = String::from("<p>");
        // output.push_str(source.trim());
        // output.push_str("</p>");

        // Some(output)

        //Some(r#"<p>This is a test run of the website builder</p>"#.to_string())
        //Some(String::from(""))

        //
    }

    pub fn content(&self) -> Option<String> {
        //let (_, b) = self.parse_content().unwrap();

        // dbg!(&self.source_data);
        // dbg!(self.content_dev2());

        Some("<p>This is a test run of the website builder</p>".to_string())
        // self.content_dev2()

        //
    }

    pub fn content_dev(&self) -> Option<String> {
        let (_, b) = self.parse_content_dev().unwrap();
        b
    }

    fn parse_content_dev(&self) -> IResult<&str, Option<String>> {
        self.parse_content_dev2()
        // let (a, _) = take_until("-> p\n\n")(self.source_data.as_ref().unwrap().as_str())?;
        // let (a, _) = tag("-> p\n\n")(a)?;
        // let (_, b) = rest(a)?;
        // let mut content = String::from("<p>");
        // content.push_str(b);
        // content.push_str("</p>");
        // Ok(("", Some(content)))
    }

    pub fn content_dev2(&self) -> Option<String> {
        let (_, b) = self.parse_content_dev2().unwrap();
        b
    }

    fn parse_content_dev2(&self) -> IResult<&str, Option<String>> {
        //let (a, b) = take_until("-> ")(self.source_data.as_ref().unwrap().as_str())?;
        // let (a, b) = tag("-> ")(a)?;

        let (a, b) = alt((
            tuple((
                tag_no_case("-> title"),
                not_line_ending,
                line_ending,
                alt((take_until("\n\n-> "), rest)),
            ))
            .map(|t| self.title_section(t.3)),
            tuple((
                tag_no_case("-> p"),
                not_line_ending,
                line_ending,
                alt((take_until("\n\n-> "), rest)),
            ))
            .map(|t| {
                self.p_section(t.3)
                // Some(String::from(
                //     r#"<p>This is a test run of the website builder</p>"#,
                // ))
            }),
        ))(self.source_data.as_ref().unwrap().as_str())?;

        Ok(("", b))

        //
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
            sf.content_dev2(),
            Some(String::from(r#"<h1 class="neo-title">Delta Hotel</h1>"#))
        );
    }

    #[test]
    pub fn test_single_paragraph() {
        let mut sf = SourceFile::new();
        let lines = vec!["-> p", "", "This is a test run of the website builder"];
        sf.source_data = Some(lines.join("\n"));
        assert_eq!(
            sf.content_dev2(),
            Some(String::from(
                "<p>This is a test run of the website builder</p>"
            ))
        );
    }

    // #[test]
    // pub fn test_multiple_section() {
    //     let mut sf = SourceFile::new();
    //     let lines = vec![
    //         "-> title",
    //         "",
    //         "Echo Foxtrot",
    //         "",
    //         "-> p",
    //         "",
    //         "Light the candle",
    //     ];
    //     sf.source_data = Some(lines.join("\n"));
    //     assert_eq!(
    //         sf.content_dev2(),
    //         Some(String::from(
    //             r#"<h1 class="neo-title">Echo Foxtrot</h1><p>Light the candle</p>"#
    //         ))
    //     );
    // }

    //
}
