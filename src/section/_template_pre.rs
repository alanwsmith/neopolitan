use crate::section::section::*;
use crate::section::section_attributes::*;
use nom::character::complete::multispace0;
use nom::IResult;

pub fn pre(source: &str) -> IResult<&str, Section> {
    let (remainder, attributes) = section_attributes(source)?;
    let (remainder, _) = multispace0(remainder)?;
    Ok((
        remainder,
        Section::PreSection {
            attributes,
            raw: Some(remainder.to_string()),
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
    pub fn basic_pre() {
        let source = ["-> pre", "", "Bring your best compass", "Cap the jar"]
            .join("\n")
            .to_string();
        let expected = Some(
            vec![
                r#"<pre>Bring your best compass"#,
                r#"Cap the jar</pre>"#,
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

    #[test]
    pub fn attributes_with_code() {
        let source = [
            "-> code",
            ">> class: alfa",
            "",
            "Bring your best compass",
            "Cap the jar",
        ]
        .join("\n")
        .to_string();
        let expected = Some(
            vec![
                r#"<pre><code class="alfa">Bring your best compass"#,
                r#"Cap the jar</code></pre>"#,
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
