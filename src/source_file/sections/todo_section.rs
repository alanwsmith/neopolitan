use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::bytes::complete::take_until;
use nom::character::complete::multispace0;
use nom::combinator::opt;
use nom::combinator::rest;
use nom::multi::many0;
use nom::sequence::delimited;
use nom::sequence::preceded;
use nom::sequence::tuple;
use nom::IResult;

pub fn todo_section<'a>(source: &'a str) -> IResult<&str, Option<String>> {
    let (_, b) = many0(tuple((
        preceded(multispace0, delimited(tag("["), opt(tag("x")), tag("]"))),
        alt((take_until("\n\n"), rest)),
    )))(source)?;
    Ok((
        "",
        Some(format!(
            "<ul>{}</ul>",
            b.iter()
                .map(|x| format!(
                    r#"<li><input type="checkbox"{}> {}</li>"#,
                    match &x.0 {
                        Some(_) => " checked",
                        None => "",
                    },
                    x.1.trim()
                ))
                .collect::<Vec<String>>()
                .join("")
        )),
    ))
}

#[cfg(test)]

mod test {
    use super::*;

    #[test]
    pub fn basic_todo() {
        let lines = ["[] alfa", "", "[] bravo", ""];
        assert_eq!(
            todo_section(lines.join("\n").as_str()).unwrap().1,
            Some(format!(
                r#"<ul><li><input type="checkbox"> alfa</li><li><input type="checkbox"> bravo</li></ul>"#
            ))
        );
    }

    #[test]
    pub fn checked_todo() {
        let lines = ["[x] alfa", "", "[] bravo"];
        assert_eq!(
            todo_section(lines.join("\n").as_str()).unwrap().1,
            Some(format!(
                r#"<ul><li><input type="checkbox" checked> alfa</li><li><input type="checkbox"> bravo</li></ul>"#
            ))
        );
    }
}
