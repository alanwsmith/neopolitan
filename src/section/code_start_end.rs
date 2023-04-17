use crate::block::block::*;
use crate::section::attributes_for_section::*;
use crate::section::section::*;
use nom::bytes::complete::tag;
use nom::multi::many0;
use nom::sequence::preceded;
use nom::IResult;

// NOTE this needs to be kept separate from the
// regular code blocks so the AST can reproduce
// the content properly. (There are ways to do
// that with the same base code, but it's
// a potential source of confusion and I
// prefer being explicit.

pub fn code_start_end(source: &str) -> IResult<&str, Section> {
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
        Section::CodeStartEndSection {
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
    fn basic_code_start_end() {
        let lines = vec![
            "-> startcode",
            "",
            "startend",
            "codeblock",
            "",
            "-> endcode",
        ]
        .join("\n");
        let source = lines.as_str();
        let expected = Wrapper::Page {
            children: Some(vec![Section::CodeStartEndSection {
                attributes: None,
                children: Some(Block::RawContent {
                    text: Some("startend\ncodeblock".to_string()),
                }),
            }]),
        };
        let result = parse(source).unwrap().1;
        assert_eq!(expected, result);
    }

    #[test]
    fn code_start_end_with_attribute() {
        let lines = vec![
            "-> startcode",
            ">> rust",
            "",
            "typed start",
            "end code",
            "",
            "-> endcode",
        ]
        .join("\n");
        let source = lines.as_str();
        let expected = Wrapper::Page {
            children: Some(vec![Section::CodeStartEndSection {
                attributes: Some(vec![SectionAttribute::Attribute {
                    key: Some("rust".to_string()),
                    value: None,
                }]),
                children: Some(Block::RawContent {
                    text: Some("typed start\nend code".to_string()),
                }),
            }]),
        };
        let result = parse(source).unwrap().1;
        assert_eq!(expected, result);
    }
}
