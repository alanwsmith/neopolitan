use crate::block::Block;
use crate::content::Content;
use crate::section::Section;
use nom::IResult;

pub fn title(source: &str) -> IResult<&str, Section> {
    Ok((
        "",
        Section::Title {
            attributes: None,
            children: Some(vec![Block::P {
                attributes: None,
                children: Some(vec![Content::Text {
                    text: Some(source.trim().to_string()),
                }]),
            }]),
        },
    ))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::section::Section;
    #[test]
    fn basic_title_response() {
        let source = "\nHello, World";
        let expected = Ok((
            "",
            Section::Title {
                attributes: None,
                children: Some(vec![Block::P {
                    attributes: None,
                    children: Some(vec![Content::Text {
                        text: Some("Hello, World".to_string()),
                    }]),
                }]),
            },
        ));
        let result = title(source);
        assert_eq!(expected, result);
    }
}
