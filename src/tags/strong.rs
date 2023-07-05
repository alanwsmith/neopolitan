use crate::tags::Snippet;
use nom::bytes::complete::is_not;
use nom::bytes::complete::tag;
use nom::bytes::complete::tag_no_case;
use nom::multi::many_till;
use nom::sequence::delimited;
use nom::sequence::preceded;
use nom::IResult;
use nom::Parser;

pub fn strong(source: &str) -> IResult<&str, Snippet> {
    let (source, text_string) =
        delimited(tag("<<"), is_not("|"), tag_no_case("|strong"))(source)?;
    let (source, attrs) = many_till(
        preceded(tag("|"), is_not("|>").map(|s: &str| s.to_string())),
        tag(">>"),
    )(source)?;
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
    use crate::tags::TagAttr;

    #[rstest]
    #[case(
        "<<alfa bravo|strong>>", 
        Ok(("", Snippet::Strong{ attrs: vec![], text: "alfa bravo".to_string() })))]
   
    #[case(
        "<<alfa bravo|strong|class: charlie delta>>", 
        Ok(("", 
            Snippet::Strong{ 
                attrs: vec![
                    TagAttr::Class(
                        vec![
                            "charlie".to_string(), "delta".to_string()
                        ]
                    )
                ],
                text: "alfa bravo".to_string() 
            })))]
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
