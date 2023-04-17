use crate::block::block::*;
use crate::section::attributes_for_section::*;
use crate::section::section::*;
use nom::bytes::complete::tag;
use nom::multi::many0;
use nom::sequence::preceded;
use nom::IResult;

pub fn html(source: &str) -> IResult<&str, Section> {
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
        Section::HTMLSection {
            attributes,
            children,
        },
    ))
}

#[cfg(test)]

mod test {

    use crate::block::block::*;
    use crate::parse::parse;
    use crate::section::attributes_for_section::*;
    use crate::section::section::*;
    use crate::wrapper::wrapper::*;

    #[test]
    fn basic_html() {
        let lines = vec![
            "-> html",
            "",
            r#"<span class="testing">html test</span>"#,
            "",
        ]
        .join("\n");
        let source = lines.as_str();
        let expected = Wrapper::Page {
            children: Some(vec![Section::HTMLSection {
                attributes: None,
                children: Some(Block::RawContent {
                    text: Some(r#"<span class="testing">html test</span>"#.to_string()),
                }),
            }]),
        };
        let result = parse(source).unwrap().1;
        assert_eq!(expected, result);
    }

    #[test]
    fn foxtrot2_attributes() {
        let lines = vec!["-> code", ">> language: rust", "", "some code", "more code"].join("\n");
        let source = lines.as_str();
        let expected = Wrapper::Page {
            children: Some(vec![Section::CodeSection {
                attributes: Some(vec![SectionAttribute::Attribute {
                    key: Some("language".to_string()),
                    value: Some("rust".to_string()),
                }]),
                children: Some(Block::RawContent {
                    text: Some("some code\nmore code".to_string()),
                }),
            }]),
        };
        let result = parse(source).unwrap().1;
        assert_eq!(expected, result);
    }
}