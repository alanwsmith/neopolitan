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
        };
        let results = aside(&source).unwrap().1;
        assert_eq!(expected, results);
    }
}
