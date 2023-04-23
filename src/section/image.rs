use crate::section::section::*;
use crate::section::section_attributes::*;
use nom::character::complete::multispace0;
use nom::IResult;

pub fn image(source: &str) -> IResult<&str, Section> {
    // dbg!(&source);
    let (remainder, attributes) = section_attributes(source)?;
    // dbg!(&remainder);
    let (remainder, _) = multispace0(remainder)?;
    // dbg!(&attributes);
    Ok((
        remainder,
        Section::ImageSection {
            attributes,
            src: Some("asdf".to_string()),
            alt_text: Some("wer".to_string()),
        },
    ))
}

#[cfg(test)]
mod test {
    use crate::parse::parse::*;
    use crate::source_file::source_file::*;
    use crate::tests::remove_whitespace::remove_whitespace;
    use crate::universe::create_env::create_env;
    use crate::universe::universe::Universe;

    #[test]
    pub fn basic_image() {
        let source = ["-> image", ">> somepath", "", "Say it slowly"]
            .join("\n")
            .to_string();
        let expected = Some(
            vec![r#"<img src="some/path" alt="Say it slowly" />"#]
                .join("\n")
                .to_string(),
        );
        let mut u = Universe::new();
        u.env = Some(create_env("./site/templates"));
        let mut sf = SourceFile::new();
        sf.raw = Some(source);
        sf.parsed = parse(sf.raw.as_ref().unwrap().as_str()).unwrap().1;
        let output = sf.output(&u);
        assert_eq!(remove_whitespace(expected), remove_whitespace(output),);
    }
}
