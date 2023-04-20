use crate::block::block::*;
use crate::section::section::*;
use crate::section::section_attributes::*;
use nom::character::complete::multispace0;
use nom::combinator::eof;
use nom::multi::many_till;
use nom::IResult;

pub fn note(source: &str) -> IResult<&str, Section> {
    let (remainder, attributes) = section_attributes(source)?;
    let (remainder, _) = multispace0(remainder)?;
    let (remainder, blocks) = many_till(block, eof)(remainder)?;
    Ok((
        remainder,
        Section::NoteSection {
            attributes,
            children: Some(blocks.0),
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
    pub fn note_with_class_and_another_attribute() {
        let source = [
            "-> note",
            ">> class: charlie",
            ">> id: oscar",
            "",
            "Mark the spot",
            "",
            "Pull the dart",
        ]
        .join("\n")
        .to_string();
        let expected = Some(
            vec![
                r#"<div class="note charlie" id="oscar">"#,
                r#"<p>Mark the spot</p>"#,
                r#"<p>Pull the dart</p>"#,
                r#"</div>"#,
            ]
            .join("\n")
            .to_string(),
        );
        let mut u = Universe::new();
        u.env = Some(create_env("./src/tests/templates"));
        let mut sf = SourceFile::new();
        sf.raw_data = Some(source);
        sf.parsed = parse(sf.raw_data.as_ref().unwrap().as_str()).unwrap().1;
        let output = sf.output(u);
        assert_eq!(remove_whitespace(expected), remove_whitespace(output),);
    }
}
