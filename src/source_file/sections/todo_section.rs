use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::bytes::complete::take_until;
use nom::character::complete::multispace0;
use nom::combinator::rest;
use nom::multi::many0;
use nom::sequence::tuple;
use nom::IResult;

pub fn todo_section<'a>(source: &'a str) -> IResult<&str, Option<String>> {
    let (a, b) = many0(tuple((
        multispace0,
        tag("[]"),
        alt((take_until("\n\n"), rest)),
    )))(source)?;
    dbg!(&b);
    dbg!(&a);

    let output: Vec<String> = b
        .iter()
        .map(|x| format!(r#"<li><input type="checkbox"> {}</li>"#, x.2.trim()))
        .collect();
    dbg!(&output);

    Ok(("", Some(format!("<ul>{}</ul>", output.join("")))))
}

#[cfg(test)]

mod test {
    use super::*;

    #[test]
    pub fn solo_basic_todo() {
        let lines = ["[] alfa", "", "[] bravo", ""];
        assert_eq!(
            todo_section(lines.join("\n").as_str()).unwrap().1,
            Some(format!(
                r#"<ul><li><input type="checkbox"> alfa</li><li><input type="checkbox"> bravo</li></ul>"#
            ))
        );
    }
}
