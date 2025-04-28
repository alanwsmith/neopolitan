use nom::IResult;
use nom::Parser;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::combinator::not;
use nom::combinator::peek;
use nom::sequence::terminated;

// PROBABLY DEPRECATED: TODO: Move into specific files
// that need it because they have slightly differnt
// requirements.

pub fn single_character(source: &str) -> IResult<&str, &str> {
    let (source, result) = alt((
        terminated(tag("`"), peek(not(tag("`")))),
        terminated(tag("~"), peek(not(tag("~")))),
        terminated(tag("!"), peek(not(tag("!")))),
        terminated(tag("@"), peek(not(tag("@")))),
        terminated(tag("#"), peek(not(tag("#")))),
        terminated(tag("$"), peek(not(tag("$")))),
        terminated(tag("%"), peek(not(tag("%")))),
        terminated(tag("^"), peek(not(tag("^")))),
        terminated(tag("*"), peek(not(tag("*")))),
        terminated(tag("["), peek(not(tag("[")))),
        terminated(tag("]"), peek(not(tag("]")))),
        terminated(tag("{"), peek(not(tag("{")))),
        terminated(tag("}"), peek(not(tag("}")))),
        terminated(tag("<"), peek(not(tag("<")))),
        terminated(tag(">"), peek(not(tag(">")))),
        terminated(tag("("), peek(not(tag("(")))),
        terminated(tag(")"), peek(not(tag(")")))),
    ))
    .parse(source)?;
    Ok((source, result))
}

#[cfg(test)]
mod test {
    use super::*;
    use pretty_assertions::assert_eq;
    use rstest::rstest;

    #[rstest]
    #[case("`", "`", "")]
    #[case("`x", "`", "x")]
    #[case("~", "~", "")]
    #[case("~x", "~", "x")]
    #[case("!", "!", "")]
    #[case("!x", "!", "x")]
    #[case("@", "@", "")]
    #[case("@x", "@", "x")]
    #[case("#", "#", "")]
    #[case("#x", "#", "x")]
    #[case("$", "$", "")]
    #[case("$x", "$", "x")]
    #[case("%", "%", "")]
    #[case("%x", "%", "x")]
    #[case("^", "^", "")]
    #[case("^x", "^", "x")]
    #[case("*", "*", "")]
    #[case("*x", "*", "x")]
    #[case("[", "[", "")]
    #[case("[x", "[", "x")]
    #[case("]", "]", "")]
    #[case("]x", "]", "x")]
    #[case("{", "{", "")]
    #[case("{x", "{", "x")]
    #[case("}", "}", "")]
    #[case("}x", "}", "x")]
    #[case("<", "<", "")]
    #[case("<x", "<", "x")]
    #[case(">", ">", "")]
    #[case(">x", ">", "x")]
    #[case("(", "(", "")]
    #[case("(x", "(", "x")]
    #[case(")", ")", "")]
    #[case(")x", ")", "x")]
    fn single_character_valid_tests(
        #[case] source: &str,
        #[case] left: &str,
        #[case] remainder: &str,
    ) {
        let right = single_character(source).unwrap();
        assert_eq!(left, right.1);
        assert_eq!(remainder, right.0);
    }

    #[rstest]
    #[case("``")]
    #[case("~~")]
    #[case("!!")]
    #[case("@@")]
    #[case("##")]
    #[case("$$")]
    #[case("%%")]
    #[case("^^")]
    #[case("&&")]
    #[case("**")]
    #[case("((")]
    #[case("))")]
    #[case("<<")]
    #[case(">>")]
    #[case("[[")]
    #[case("]]")]
    #[case("{{")]
    #[case("}}")]
    fn single_character_invalid_tests(#[case] source: &str) {
        let result = single_character(source);
        match result {
            Ok(value) => {
                dbg!(value);
                assert!(false)
            }
            Err(_) => assert!(true),
        }
    }
}
