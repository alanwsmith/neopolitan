use nom::IResult;

#[derive(Debug, PartialEq)]
pub enum Snippet {
    Plain { text: Option<String> },
}

pub fn snippet(source: &str) -> IResult<&str, Snippet> {
    // This is for individaul sections of a block like
    // raw plain text, code, strong, links, etc...
    // dbg!(source);
    Ok((
        "",
        Snippet::Plain {
            text: Some(source.to_string()),
        },
    ))
}

#[cfg(test)]
mod test {
    use crate::snippet::snippet::*;

    #[test]
    pub fn basic_test() {
        let expected = Snippet::Plain {
            text: Some("Take the winding path".to_string()),
        };
        let result = snippet("Take the winding path").unwrap().1;
        assert_eq!(expected, result);
    }
}
