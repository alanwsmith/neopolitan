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
    use super::*;
    use crate::snippet::snippet_enum::*;
    // use crate::section::section::*;
    // use crate::section::section_attributes::*;

    // use crate::parse::parse::*;
    // use crate::source_file::source_file::*;
    // use crate::tests::remove_whitespace::remove_whitespace;
    // use crate::universe::create_env::create_env;
    // use crate::universe::universe::Universe;

    #[test]
    pub fn two_attribute_on_aside() {
        let source = [">> class: delta", ">> id: bravo", "", "Run the race"]
            .join("\n")
            .to_string();

        let expected = Section::AsideSection {
            attributes: Some(vec![
                SectionAttribute::Attribute {
                    key: Some("class".to_string()),
                    value: Some("delta".to_string()),
                },
                SectionAttribute::Attribute {
                    key: Some("id".to_string()),
                    value: Some("bravo".to_string()),
                },
            ]),
            children: Some(vec![Block::Text {
                snippets: Some(vec![Snippet::Plain {
                    text: Some("Run the race".to_string()),
                }]),
            }]),
            // attributes_string: Some(r#" title="Some new title""#.to_string()),
        };
        let results = aside(&source).unwrap().1;
        assert_eq!(expected, results);

        // let expected = Some(
        //     vec![
        //         r#"<aside class="delta" id="bravo">"#,
        //         r#"<p>Hold the hammer</p>"#,
        //         r#"<p>Heave the line</p>"#,
        //         r#"</aside>"#,
        //     ]
        //     .join("\n")
        //     .to_string(),
        // );
        // let mut u = Universe::new();
        // u.env = Some(create_env("./site/templates"));
        // let mut sf = SourceFile::new();
        // sf.raw = Some(source);
        // sf.parsed = parse(sf.raw.as_ref().unwrap().as_str()).unwrap().1;
        // let output = sf.output(&u);
        // assert_eq!(remove_whitespace(expected), remove_whitespace(output),);
    }
}
