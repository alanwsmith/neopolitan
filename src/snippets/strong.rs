use crate::snippets::Snippet;
use nom::bytes::complete::is_not;
use nom::bytes::complete::tag;
use nom::bytes::complete::tag_no_case;
use nom::sequence::delimited;
use nom::IResult;

pub fn strong(source: &str) -> IResult<&str, Snippet> {
    let (source, text_string) =
        delimited(tag("<<"), is_not("|"), tag_no_case("|strong"))(source)?;
    Ok((
        source,
        Snippet::Strong {
            text: text_string.to_string(),
            attrs: vec![],
        },
    ))
}



#[cfg(test)]
mod test{
    use super::*;

    #[test]
    pub fn solo_basic_strong() {
        let line = "<<alfa|strong>>";
        let expected = Snippet::Strong{ attrs: vec![], text: "alfa".to_string() };
        assert_eq!(expected, strong(line).unwrap().1);
    }
}
