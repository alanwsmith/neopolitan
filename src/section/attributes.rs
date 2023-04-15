use crate::block::block::*;
use crate::section::attributes_for_section::*;
use crate::section::section::*;
use nom::bytes::complete::tag;
use nom::multi::many0;
use nom::sequence::preceded;
use nom::IResult;

pub fn attributes(source: &str) -> IResult<&str, Section> {
    // dbg!(source);
    let (source, att_capture) = many0(preceded(tag(">> "), section_attribute))(source).unwrap();
    let attributes = if att_capture.is_empty() {
        None
    } else {
        Some(att_capture)
    };
    let children = if source.trim().is_empty() {
        None
    } else {
        Some(Block::RawContent {
            text: Some(source.trim().to_string()),
        })
    };
    Ok((
        source,
        Section::AttributesSection {
            attributes,
            children,
        },
    ))
}

#[cfg(test)]
mod tests {
    use crate::block::block::*;
    use crate::parse::parse;
    use crate::section::attributes_for_section::*;
    use crate::section::section::*;
    use crate::wrapper::wrapper::*;

    #[test]
    fn basic_attributes() {
        let lines = vec!["-> attributes", ">> somekey: some value"].join("\n");
        let source = lines.as_str();
        let expected = Wrapper::Page {
            children: Some(vec![Section::AttributesSection {
                attributes: Some(vec![
                    (SectionAttribute::Attribute {
                        key: Some("somekey".to_string()),
                        value: Some("some value".to_string()),
                    }),
                ]),
                children: None,
            }]),
        };
        let result = parse(source).unwrap().1;
        assert_eq!(expected, result);
    }

    #[test]
    fn attributes_with_content() {
        let lines = vec![
            "-> attributes",
            ">> somekey: some value",
            "",
            "attribute content",
        ]
        .join("\n");
        let source = lines.as_str();
        let expected = Wrapper::Page {
            children: Some(vec![Section::AttributesSection {
                attributes: Some(vec![
                    (SectionAttribute::Attribute {
                        key: Some("somekey".to_string()),
                        value: Some("some value".to_string()),
                    }),
                ]),
                children: Some(Block::RawContent {
                    text: Some("attribute content".to_string()),
                }),
            }]),
        };
        let result = parse(source).unwrap().1;
        assert_eq!(expected, result);
    }
}
