use nom::IResult;
use nom::Parser;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::combinator::not;
use nom::combinator::peek;
use nom::sequence::terminated;

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

    //
}

// #[cfg(test)]
// mod test {
//     use super::*;
//     use pretty_assertions::assert_eq;
//     use rstest::rstest;
//     #[rstest]
//     #[case("|alfa", "alfa", "")]
//     #[case("|alfa|", "alfa", "|")]
//     #[case("| alfa", "alfa", "")]
//     #[case("| alfa |", "alfa", "|")]
//     #[case("| alfa-bravo", "alfa-bravo", "")]
//     #[case("|http://www.example.com/", "http://www.example.com/", "")]
//     #[case("|alfa`", "alfa`", "")]
//     #[case("|alfa``", "alfa", "``")]
//     #[case("|alfa\\``", "alfa``", "")]
//     #[case("|alfa::bravo", "alfa::bravo", "")]
//     #[case("|alfa[[bravo]]", "alfa[[bravo]]", "")]
//     #[case("|\nalfa|", "alfa", "|")]
//     #[case("|alfa\n|", "alfa", "|")]
//     #[case("| \n alfa \n |", "alfa", "|")]
//     #[case("|alfa\\|", "alfa|", "")]
//     fn span_flag_valid_tests(
//         #[case] source: &str,
//         #[case] found: &str,
//         #[case] remainder: &str,
//     ) {
//         let characters = "%@~*^![]{}<>_#:".to_string();
//         let left = RawShorthandMetadata::Flag(found.to_string());
//         let right = span_flag(source, characters).unwrap();
//         assert_eq!(left, right.1);
//         assert_eq!(remainder, right.0);
//     }
//     #[rstest]
//     #[case("alfa")]
//     fn span_flag_invalid_tests(#[case] source: &str) {
//         let characters = "%@~*^![]{}<>_#:".to_string();
//         let result = span_flag(source, characters);
//         match result {
//             Ok(_) => assert!(false),
//             Err(_) => assert!(true),
//         }
//     }
// }
