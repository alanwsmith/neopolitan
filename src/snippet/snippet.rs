use nom::IResult;

#[derive(Debug, PartialEq)]
pub enum Snippet {
    Plain { text: Option<String> },
}

pub fn snippet(source: &str) -> IResult<&str, Snippet> {
    // This is for individaul sections of a block like
    // raw plain text, code, strong, links, etc...
    dbg!(source);
    Ok((
        "",
        Snippet::Plain {
            text: Some(source.to_string()),
        },
    ))
}
