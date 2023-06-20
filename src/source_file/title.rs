#![allow(warnings)]
use crate::source_file::source_file::SourceFile;
use nom::bytes::complete::tag;
use nom::bytes::complete::take_until;
use nom::character::complete::multispace1;
use nom::IResult;

impl SourceFile {
    pub fn title(&self) -> Option<String> {
        // Some(String::from("Alfa Bravo"))

        self.title_dev()
    }

    pub fn title_dev(&self) -> Option<String> {
        let (a, b) = self.parse(&self.source_data.as_ref().unwrap()).unwrap();
        Some(b)
    }

    pub fn parse<'a>(&'a self, source: &'a str) -> IResult<&str, String> {
        let (a, b) = take_until("-> title")(source)?;
        let (a, b) = tag("-> title")(a)?;
        let (a, b) = multispace1(a)?;
        let (a, b) = take_until("\n->")(a)?;
        Ok((a, b.trim().to_string()))
    }
}

#[cfg(test)]

mod test {

    use crate::source_file::source_file::SourceFile;

    #[test]
    pub fn title_with_other_sections() {
        let mut sf = SourceFile::new();
        let lines = vec![
            "-> title",
            "",
            "Foxtrot Golf",
            "",
            "-> p",
            "",
            "Hotel India",
        ];
        sf.source_data = Some(lines.join("\n"));
        assert_eq!(Some(String::from("Foxtrot Golf")), sf.title_dev());
    }
}
