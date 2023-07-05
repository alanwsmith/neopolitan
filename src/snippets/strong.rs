use crate::snippets::Snippet;
use nom::bytes::complete::is_not;
use nom::bytes::complete::tag;
use nom::bytes::complete::tag_no_case;
use nom::sequence::delimited;
use nom::IResult;

pub fn strong(source: &str) -> IResult<&str, Snippet> {
    let (source, text_string) =
        delimited(tag("<<"), is_not("|"), tag_no_case("|strong"))(source)?;
    let (source, _) = tag(">>")(source)?;
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
    use rstest::rstest;
    use nom::error::Error;
    use nom::Err;


    #[rstest]
    #[case("<<alfa|strong>>", Ok(("", Snippet::Strong{ attrs: vec![], text: "alfa".to_string() })))]
    fn solo_strong_runner(#[case] input: &str, #[case] expected: Result<(&str, Snippet), Err<Error<&str>>>) {
        assert_eq!(expected, strong(input))
    }



    // #[test]
    // pub fn solo_basic_strong() {
    //     let line = "<<alfa|strong>>";
    //     let expected = Snippet::Strong{ attrs: vec![], text: "alfa".to_string() };
    //     assert_eq!(expected, strong(line))));
    // }


    // #[test]
    // pub fn solo_basic_strong() {
    //     let line = "<<alfa|strong>>";
    //     let expected = Snippet::Strong{ attrs: vec![], text: "alfa".to_string() };
    //     assert_eq!(expected, strong(line).unwrap().1);
    // }






}
