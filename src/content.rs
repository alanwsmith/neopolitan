use crate::text::text;
use nom::IResult;

#[derive(Debug, PartialEq)]
pub enum Content {
    Text { value: Option<String> },
}

pub fn content(source: &str) -> IResult<&str, Vec<Content>> {
    let (a, b) = text(source)?;
    Ok((a, vec![b]))
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn basic_content() {
        let source = "alfa bravo";
        let expected = Ok((
            "",
            vec![Content::Text {
                value: Some("alfa bravo".to_string()),
            }],
        ));
        let result = content(source);
        assert_eq!(expected, result);
    }
}
