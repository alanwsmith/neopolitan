use crate::section::section::*;
use crate::section::section_attributes::*;
use nom::bytes::complete::tag;
use nom::character::complete::line_ending;
use nom::character::complete::multispace1;
use nom::character::complete::not_line_ending;
use nom::IResult;

pub fn image(source: &str) -> IResult<&str, Section> {
    let (remainder, _) = tag(">>")(source)?;
    let (remainder, _) = multispace1(remainder)?;
    let (remainder, src) = not_line_ending(remainder)?;
    let (remainder, _) = line_ending(remainder)?;
    let (remainder, attributes) = section_attributes(remainder)?;
    Ok((
        "",
        Section::ImageSection {
            attributes,
            src: Some(src.trim().to_string()),
            alt_text: Some(remainder.trim().to_string()),
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
        let source = [
            "-> image",
            ">> http://placekitten.com/g/200/300",
            "",
            "Say it slowly",
        ]
        .join("\n")
        .to_string();
        let expected = Some(
            vec![r#"<img src="http://placekitten.com/g/200/300" alt="Say it slowly" />"#]
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

    #[test]
    pub fn image_with_attributes() {
        let source = [
            "-> image",
            ">> http://placekitten.com/g/200/300",
            ">> class: romeo",
            "",
            "Say it slowly",
        ]
        .join("\n")
        .to_string();
        let expected = Some(
            vec![r#"<img src="http://placekitten.com/g/200/300" alt="Say it slowly" class="romeo" />"#]
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
