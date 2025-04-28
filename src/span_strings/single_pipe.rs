use nom::IResult;
use nom::Parser;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::combinator::not;
use nom::combinator::peek;
use nom::sequence::terminated;

pub fn single_pipe(source: &str) -> IResult<&str, &str> {
    let (source, result) =
        alt((terminated(tag("|"), peek(not(tag("|")))),)).parse(source)?;
    Ok((source, result))
}

#[cfg(test)]
mod test {
    use super::*;
    use pretty_assertions::assert_eq;
    use rstest::rstest;

    #[rstest]
    #[case("|", "|", "")]
    #[case("|x", "|", "x")]
    fn single_pipe_valid_tests(
        #[case] source: &str,
        #[case] left: &str,
        #[case] remainder: &str,
    ) {
        let right = single_pipe(source).unwrap();
        assert_eq!(left, right.1);
        assert_eq!(remainder, right.0);
    }

    #[rstest]
    #[case("||")]
    fn single_pipe_invalid_tests(#[case] source: &str) {
        let result = single_pipe(source);
        match result {
            Ok(value) => {
                dbg!(value);
                assert!(false)
            }
            Err(_) => assert!(true),
        }
    }
}
