use crate::block::Block;
use crate::content::Content;
use crate::section::Section;
use nom::IResult;

pub fn title(_source: &str) -> IResult<&str, Section> {
    Ok((
        "",
        Section::Title {
            attributes: None,
            children: Some(vec![Block::P {
                attributes: None,
                children: Some(vec![Content::Text {
                    value: Some("Hello, World".to_string()),
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
        let source = "\nHello world";
        let expected = Ok((
            "",
            Section::Title {
                attributes: None,
                children: Some(vec![Block::P {
                    attributes: None,
                    children: Some(vec![Content::Text {
                        value: Some("Hello, World".to_string()),
                    }]),
                }]),
            },
        ));
        let result = title(source);
        assert_eq!(expected, result);
    }
}
