use crate::section::section;
use crate::section::Section;
use nom::combinator::eof;
use nom::multi::many_till;
use nom::IResult;

#[derive(Debug, PartialEq)]
pub enum Wrapper {
    Page { children: Option<Vec<Section>> },
}

pub fn wrapper(source: &str) -> IResult<&str, Wrapper> {
    let (_, sections) = many_till(section, eof)(source)?;
    let response = Wrapper::Page {
        children: Some(sections.0),
    };
    Ok(("", response))
}

#[cfg(test)]
mod test {

    use super::*;
    use crate::block::Block;
    use crate::content::Content;

    #[test]
    fn basic_page_test() {
        let source = "-> title\n\nHello, Neopolitan";
        let expected = Ok((
            "",
            Wrapper::Page {
                children: Some(vec![Section::Title {
                    attributes: None,
                    children: Some(vec![Block::P {
                        attributes: None,
                        children: Some(vec![Content::Text {
                            text: Some("Hello, Neopolitan".to_string()),
                        }]),
                    }]),
                }]),
            },
        ));
        let result = wrapper(source);
        assert_eq!(expected, result);
    }
}
