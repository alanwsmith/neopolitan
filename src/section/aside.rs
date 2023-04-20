use crate::block::block::*;
use crate::section::section::*;
use crate::section::section_attributes::*;
use nom::character::complete::multispace0;
use nom::combinator::eof;
use nom::multi::many_till;
use nom::IResult;

pub fn aside(source: &str) -> IResult<&str, Section> {
    let (remainder, attributes) = section_attributes(source)?;
    let (remainder, _) = multispace0(remainder)?;
    let (remainder, blocks) = many_till(block, eof)(remainder)?;
    Ok((
        remainder,
        Section::AsideSection {
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
    pub fn aside_with_class_and_another_attribute() {
        let source = [
            "-> aside",
            ">> class: delta",
            ">> id: bravo",
            "",
            "Hold the hammer",
            "",
            "Heave the line",
        ]
        .join("\n")
        .to_string();
        let expected = Some(
            vec![
                r#"<aside class="delta" id="bravo">"#,
                r#"<p>Hold the hammer</p>"#,
                r#"<p>Heave the line</p>"#,
                r#"</aside>"#,
            ]
            .join("\n")
            .to_string(),
        );
        let mut u = Universe::new();
        u.env = Some(create_env("./src/tests/templates"));
        let mut sf = SourceFile::new();
        sf.raw_data = Some(source);
        sf.parsed = parse(sf.raw_data.as_ref().unwrap().as_str()).unwrap().1;
        let output = sf.output(&u);
        assert_eq!(remove_whitespace(expected), remove_whitespace(output),);
    }
}
