use crate::content::content::*;
use nom::bytes::complete::tag;
use nom::bytes::complete::take_until;
use nom::IResult;

// TODO: Remvoe `source:` field, probably.

pub fn link(source: &str) -> IResult<&str, Content> {
    let (source, url) = take_until("|")(source)?;
    let (source, _) = tag("|")(source)?;
    let (source, text) = take_until(">>")(source)?;
    let (source, _) = tag(">>")(source)?;
    Ok((
        source,
        Content::Link {
            source: None,
            attributes: None,
            url: Some(url.to_string()),
            text: Some(text.to_string()),
        },
    ))
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::content::content::*;

    #[test]
    fn basic_link() {
        let source = "https://alfa.example.com/|bravo>>";
        let expected = Ok((
            "",
            Content::Link {
                source: None,
                attributes: None,
                url: Some("https://alfa.example.com/".to_string()),
                text: Some("bravo".to_string()),
            },
        ));
        let result = link(source);
        assert_eq!(expected, result);
    }
}
