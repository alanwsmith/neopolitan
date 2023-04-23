use crate::section::section::*;
use crate::section::section_attributes::*;
use nom::bytes::complete::tag;
use nom::character::complete::line_ending;
use nom::character::complete::multispace1;
use nom::character::complete::not_line_ending;
use nom::IResult;

pub fn youtube(source: &str) -> IResult<&str, Section> {
    let (remainder, _) = tag(">>")(source)?;
    let (remainder, _) = multispace1(remainder)?;
    let (remainder, id) = not_line_ending(remainder)?;
    let (remainder, _) = line_ending(remainder)?;
    let (_, attributes) = section_attributes(remainder)?;
    Ok((
        "",
        Section::YouTubeSection {
            attributes,
            id: Some(id.trim().to_string()),
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
    pub fn basic_youtube() {
        let source = ["-> youtube", ">> gnUYoQ1pwes", ""].join("\n").to_string();
        let expected = Some(
            vec![r#"<div><iframe width="560" height="315" src="https://www.youtube.com/embed/gnUYoQ1pwes" title="YouTube video player" frameborder="0" allow="accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture; web-share" allowfullscreen></iframe></div>"#]
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
    pub fn youtube_with_attributes() {
        let source = [
            "-> youtube",
            ">> gnUYoQ1pwes",
            ">> class: delta",
            "",
            "Say it slowly",
        ]
        .join("\n")
        .to_string();
        let expected = Some(
            vec![r#"<div class="delta"><iframe width="560" height="315" src="https://www.youtube.com/embed/gnUYoQ1pwes" title="YouTube video player" frameborder="0" allow="accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture; web-share" allowfullscreen></iframe></div>"#]
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
