use nom::branch::alt;
use nom::bytes::complete::tag_no_case;
use nom::bytes::complete::take_until;
use nom::character::complete::line_ending;
use nom::character::complete::not_line_ending;
use nom::combinator::eof;
use nom::sequence::delimited;
use nom::sequence::tuple;
use nom::IResult;

pub fn template(source: &str) -> IResult<&str, String> {
    let (source, _) = take_until("-> attributes")(source)?;
    let (source, template) = delimited(
        tuple((take_until(">> type: "), tag_no_case(">> type: "))),
        not_line_ending,
        alt((line_ending, eof)),
    )(source)?;
    let return_value = format!("{}.j2", template.trim());

    Ok((source, return_value))
}

#[cfg(test)]

mod test {

    use super::*;

    #[test]
    pub fn basic_test() {
        let lines = vec!["-> attributes", ">> type: alfa"].join("\n");
        let expected = "alfa.j2".to_string();
        assert_eq!(expected, template(lines.as_str()).unwrap().1);
    }

    #[test]
    pub fn basic_with_whitespace_problem() {
        let lines = vec!["-> attributes", ">> type: bravo "].join("\n");
        let expected = "bravo.j2".to_string();
        assert_eq!(expected, template(lines.as_str()).unwrap().1);
    }
}
