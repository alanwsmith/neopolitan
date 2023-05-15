use crate::block::block::*;
use crate::section::section::*;
use crate::section::section_attributes::*;
use nom::character::complete::multispace0;
use nom::combinator::eof;
use nom::multi::many_till;
use nom::IResult;

pub fn subtitle(source: &str) -> IResult<&str, Section> {
    let (remainder, attributes) = section_attributes(source)?;
    let (remainder, _) = multispace0(remainder)?;
    let (remainder, blocks) = many_till(block, eof)(remainder)?;
    Ok((
        remainder,
        Section::SubtitleSection {
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
    pub fn single_line() {
        let source = ["-> subtitle", "", "Pour the stew"].join("\n").to_string();
        let expected = Some(
            vec![r#"<div class="subtitle">Pour the stew</div>"#]
                .join("\n")
                .to_string(),
        );
        let mut u = Universe::new();
        u.env = Some(create_env("./templates"));
        let mut sf = SourceFile::new();
        sf.raw = Some(source);
        sf.parsed = parse(sf.raw.as_ref().unwrap().as_str()).unwrap().1;
        let output = sf.output(&u);
        assert_eq!(remove_whitespace(expected), remove_whitespace(output),);
    }

    #[test]
    pub fn multiple_lines() {
        let source = ["-> subtitle", "", "Pitch the straw", "", "through the door"]
            .join("\n")
            .to_string();
        let expected = Some(
            vec![
                r#"<div class="subtitle">Pitch the straw</div>"#,
                "<p>through the door</p>",
            ]
            .join("\n")
            .to_string(),
        );
        let mut u = Universe::new();
        u.env = Some(create_env("./templates"));
        let mut sf = SourceFile::new();
        sf.raw = Some(source);
        sf.parsed = parse(sf.raw.as_ref().unwrap().as_str()).unwrap().1;
        let output = sf.output(&u);
        assert_eq!(remove_whitespace(expected), remove_whitespace(output),);
    }

    #[test]
    pub fn single_attribute() {
        let source = ["-> subtitle", ">> id: tango", "", "Pack your kits"]
            .join("\n")
            .to_string();
        let expected = Some(
            vec![r#"<div id="tango" class="subtitle">Pack your kits</div>"#]
                .join("\n")
                .to_string(),
        );
        let mut u = Universe::new();
        u.env = Some(create_env("./templates"));
        let mut sf = SourceFile::new();
        sf.raw = Some(source);
        sf.parsed = parse(sf.raw.as_ref().unwrap().as_str()).unwrap().1;
        let output = sf.output(&u);
        assert_eq!(remove_whitespace(expected), remove_whitespace(output),);
    }

    #[test]
    pub fn class_attribute() {
        let source = ["-> subtitle", ">> class: hotel", "", "Pick a card"]
            .join("\n")
            .to_string();
        let expected = Some(
            vec![r#"<div class="subtitle hotel">Pick a card</div>"#]
                .join("\n")
                .to_string(),
        );
        let mut u = Universe::new();
        u.env = Some(create_env("./templates"));
        let mut sf = SourceFile::new();
        sf.raw = Some(source);
        sf.parsed = parse(sf.raw.as_ref().unwrap().as_str()).unwrap().1;
        let output = sf.output(&u);
        assert_eq!(remove_whitespace(expected), remove_whitespace(output),);
    }

    #[test]
    pub fn class_and_another_attribute() {
        let source = [
            "-> subtitle",
            ">> class: victor",
            ">> id: bravo",
            "",
            "Open the crate",
        ]
        .join("\n")
        .to_string();
        let expected = Some(
            vec![r#"<div id="bravo" class="subtitle victor">Open the crate</div>"#]
                .join("\n")
                .to_string(),
        );
        let mut u = Universe::new();
        u.env = Some(create_env("./templates"));
        let mut sf = SourceFile::new();
        sf.raw = Some(source);
        sf.parsed = parse(sf.raw.as_ref().unwrap().as_str()).unwrap().1;
        let output = sf.output(&u);
        assert_eq!(remove_whitespace(expected), remove_whitespace(output),);
    }
}
