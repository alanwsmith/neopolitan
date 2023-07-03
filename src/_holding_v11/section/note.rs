use crate::block::block::*;
use crate::section::section::*;
use crate::section::lib::get_title_from_attributes::get_title_from_attributes;
use crate::section::section_attributes::*;
use nom::character::complete::multispace0;
use nom::combinator::eof;
use nom::multi::many_till;
use nom::IResult;

pub fn note(source: &str) -> IResult<&str, Section> {
    let (remainder, attributes) = section_attributes(source)?;
    let (remainder, _) = multispace0(remainder)?;
    let (remainder, blocks) = many_till(block, eof)(remainder)?;
    let title = get_title_from_attributes(&attributes);
    Ok((
        remainder,
        Section::NoteSection {
            attributes,
            children: Some(blocks.0),
            title,
        },
    ))
}

#[cfg(test)]
mod test {
    use crate::block::block::*;
    use crate::section::note::note;
    use crate::section::section::*;
    use crate::section::section_attributes::*;
    use crate::snippet::snippet_enum::*;

    // TODO: Add test with title

    #[test]
    pub fn named_class_on_note() {
        let source = [
            ">> class: delta",
            ">> id: bravo",
            "",
            "Lift the stone",
            "",
            "Fasten two pins",
        ]
        .join("\n")
        .to_string();
        let expected = Section::NoteSection {
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
            children: Some(vec![
                Block::Text {
                    snippets: Some(vec![Snippet::Plain {
                        text: Some("Lift the stone".to_string()),
                    }]),
                },
                Block::Text {
                    snippets: Some(vec![Snippet::Plain {
                        text: Some("Fasten two pins".to_string()),
                    }]),
                },
            ]),
            title: None,
        };
        let results = note(&source).unwrap().1;
        assert_eq!(expected, results);
    }
}
