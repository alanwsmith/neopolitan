use crate::section::section::*;
use crate::section::section_attributes::*;
// use nom::bytes::complete::tag;
// use nom::character::complete::line_ending;
// use nom::character::complete::multispace1;
// use nom::character::complete::not_line_ending;
use nom::IResult;

pub fn object(source: &str) -> IResult<&str, Section> {
    let (remainder, attributes) = section_attributes(source)?;
    Ok((remainder, Section::ObjectSection { attributes }))
}

#[cfg(test)]
mod test {
    use crate::parse::parse::*;
    use crate::source_file::source_file::*;
    use crate::tests::remove_whitespace::remove_whitespace;
    use crate::universe::create_env::create_env;
    use crate::universe::universe::Universe;

    #[ignore]
    #[test]
    pub fn basic_object() {
        let source = [
            "-> object",
            ">> type: application/pdf",
            ">> data: https://www.example.com/example.pdf",
            "",
        ]
        .join("\n")
        .to_string();
        let expected = Some(
            vec![
                r#"<object"#,
                r#"type="application/pdf""#,
                r#"data="https://www.example.com/example.pdf""#,
                r#"></object>"#,
            ]
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

    // #[test]
    // pub fn image_with_attributes() {
    //     let source = [
    //         "-> image",
    //         ">> http://placekitten.com/g/200/300",
    //         ">> class: romeo",
    //         "",
    //         "Say it slowly",
    //     ]
    //     .join("\n")
    //     .to_string();
    //     let expected = Some(
    //         vec![r#"<img src="http://placekitten.com/g/200/300" alt="Say it slowly" class="romeo" />"#]
    //             .join("\n")
    //             .to_string(),
    //     );
    //     let mut u = Universe::new();
    //     u.env = Some(create_env("./site/templates"));
    //     let mut sf = SourceFile::new();
    //     sf.raw = Some(source);
    //     sf.parsed = parse(sf.raw.as_ref().unwrap().as_str()).unwrap().1;
    //     let output = sf.output(&u);
    //     assert_eq!(remove_whitespace(expected), remove_whitespace(output),);
    // }
}
