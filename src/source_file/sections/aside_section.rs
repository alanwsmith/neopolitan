use nom::branch::alt;
use nom::bytes::complete::take_until;
use nom::character::complete::multispace0;
use nom::combinator::rest;
// use nom::multi::many0;
use nom::combinator::eof;
use nom::multi::many_till;
use nom::sequence::preceded;
use nom::IResult;

pub fn aside_section<'a>(
    source: &'a str,
) -> IResult<&str, Option<String>> {
    let (a, b) = many_till(
        preceded(
            multispace0,
            alt((take_until("\n\n"), rest)),
        ),
        eof,
    )(source)?;
    dbg!(&b);
    dbg!(&a);
    Ok(("", Some(format!("{}", ""))))
}

#[cfg(test)]

mod test {
    use super::*;

    #[test]
    #[ignore]
    pub fn test_basic_aside() {
        let lines = vec![
            "",
            "Slide the tray across the glass top.",
        ];
        assert_eq!(
            aside_section(lines.join("").as_str())
                .unwrap()
                .1,
            Some(format!("{}", r#"<aside>"#))
        );
    }
}
