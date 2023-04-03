use crate::content::Content;
use nom::IResult;

pub fn text(source: &str) -> IResult<&str, Content> {
    Ok((
        "",
        Content::Text {
            value: Some(source.to_string()),
        },
    ))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::content::Content;
    #[test]
    fn basic_title_response() {
        let source = "Here it is";
        let expected = Ok((
            "",
            Content::Text {
                value: Some("Here it is".to_string()),
            },
        ));
        let result = text(source);
        assert_eq!(expected, result);
    }
}
