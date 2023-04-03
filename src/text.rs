use crate::content::Content;
use nom::IResult;

pub fn text(source: &str) -> IResult<&str, Content> {
    Ok((
        "",
        Content::Text {
            text: Some(source.to_string()),
        },
    ))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::content::Content;
    #[test]
    fn basic_text_response() {
        let source = "Here it is";
        let expected = Ok((
            "",
            Content::Text {
                text: Some("Here it is".to_string()),
            },
        ));
        let result = text(source);
        assert_eq!(expected, result);
    }
}
