use nom::IResult;

pub fn aside_section<'a>(source: &'a str) -> IResult<&str, Option<String>> {
    let (a, b) = many1(preceded(multispace0, alt((take_until("\n\n"), rest))))(source)?;
    Ok(("", Some(format!("{}", ""))))
}

#[cfg(test)]

mod test {
    use super::*;

    #[test]
    pub fn solo_test_basic_aside() {
        let lines = vec!["", "Slide the tray across the glass top."];
        assert_eq!(
            aside_section(lines.join("").as_str()).unwrap().1,
            Some(format!("{}", ""))
        );
    }
}
