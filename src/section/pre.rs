use crate::block::block::*;
use crate::section::attributes_for_section::*;
use crate::section::section::*;
use nom::bytes::complete::tag;
use nom::multi::many0;
use nom::sequence::preceded;
use nom::IResult;

pub fn pre(source: &str) -> IResult<&str, Section> {
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
        Section::PreSection {
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
    fn basic_widget() {
        let lines = vec!["-> pre", ">> key: value", "pre content"].join("\n");
        let source = lines.as_str();
        let expected = Wrapper::Page {
            children: Some(vec![Section::PreSection {
                attributes: Some(vec![SectionAttribute::Attribute {
                    key: Some("key".to_string()),
                    value: Some("value".to_string()),
                }]),
                children: Some(Block::RawContent {
                    text: Some("pre content".to_string()),
                }),
            }]),
        };
        let result = parse(source).unwrap().1;
        assert_eq!(expected, result);
    }
}
