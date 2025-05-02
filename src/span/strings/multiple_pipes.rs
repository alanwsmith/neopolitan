use nom::IResult;
use nom::bytes::complete::is_a;

pub fn multiple_pipes(source: &str) -> IResult<&str, &str> {
    let (source, result) = is_a("|")(source)?;
    Ok((source, result))
}

#[cfg(test)]
mod test {
    use super::*;
    use pretty_assertions::assert_eq;
    use rstest::rstest;

    #[rstest]
    #[case("|", "|", "")]
    #[case("||", "||", "")]
    #[case("||||x", "||||", "x")]
    fn multiple_pipes_valid_tests(
        #[case] source: &str,
        #[case] left: &str,
        #[case] remainder: &str,
    ) {
        let right = multiple_pipes(source).unwrap();
        assert_eq!(left, right.1);
        assert_eq!(remainder, right.0);
    }
}
