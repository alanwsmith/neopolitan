use crate::section::list_item::*;
use crate::section::section::*;
use crate::section::section_attributes::*;
use nom::character::complete::multispace0;
use nom::combinator::eof;
use nom::multi::many_till;
use nom::IResult;

pub fn nav(source: &str) -> IResult<&str, Section> {
    let (remainder, attributes) = section_attributes(source)?;
    let (remainder, _) = multispace0(remainder)?;
    let (remainder, items) = many_till(list_item, eof)(remainder)?;
    Ok((
        remainder,
        Section::NavSection{
            attributes,
            children: Some(items.0),
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
    pub fn basic_nav() {
        let source = ["-> nav", "", "- alfa", "", "- bravo"]
            .join("\n")
            .to_string();
        let expected = Some(
            vec![
                r#"<nav>"#,
                r#"<ul>"#,
                r#"<li><p>alfa</p></li>"#,
                r#"<li><p>bravo</p></li>"#,
                r#"</ul>"#,
                r#"</nav>"#,
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
}
