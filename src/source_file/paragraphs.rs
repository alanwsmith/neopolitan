use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::anychar;
use nom::character::complete::multispace0;
use nom::character::complete::space0;
use nom::combinator::eof;
use nom::multi::many_till;
use nom::sequence::tuple;
use nom::IResult;
use nom::Parser;

pub fn paragraphs(
    source: &str,
) -> IResult<&str, Vec<String>> {
    let (a, b) = many_till(
        many_till(
            anychar
                .map(|x| if x == '\n' { ' ' } else { x }),
            alt((
                tuple((
                    tag("\n"),
                    space0,
                    tag("\n"),
                    multispace0,
                ))
                .map(|_| ""),
                eof,
            )),
        )
        .map(|(x, y)| (x.iter().collect::<String>(), y)),
        alt((
            tuple((
                tag("\n"),
                space0,
                tag("\n"),
                multispace0,
            ))
            .map(|_| ""),
            eof,
        )),
    )(source.trim())?;
    let results: Vec<_> =
        b.clone().0.into_iter().map(|x| x.0).collect();
    Ok((a, results))
}

#[cfg(test)]

mod test {
    use super::*;

    // #[test]
    // pub fn empty_text() {
    //     let source = vec![""].join("\n");
    //     let expected: Vec<&str> = vec![];
    //     assert_eq!(
    //         paragraphs(source.as_str()).unwrap().1,
    //         expected
    //     );
    // }

    // #[test]
    // pub fn single_line() {
    //     let source = vec!["Cap the jar"].join("\n");
    //     assert_eq!(
    //         paragraphs(source.as_str()).unwrap().1,
    //         vec!["Cap the jar"]
    //     );
    // }

    // #[test]
    // pub fn multi_line() {
    //     let source =
    //         vec!["Heave the line", "", "Heavy black lines"]
    //             .join("\n");
    //     assert_eq!(
    //         paragraphs(source.as_str()).unwrap().1,
    //         vec!["Heave the line", "Heavy black lines"]
    //     );
    // }

    // #[test]
    // pub fn three_lines() {
    //     let source = vec![
    //         "Drive the nail",
    //         "",
    //         "Open your book",
    //         "",
    //         "Tack it down",
    //     ]
    //     .join("\n");
    //     assert_eq!(
    //         paragraphs(source.as_str()).unwrap().1,
    //         vec![
    //             "Drive the nail",
    //             "Open your book",
    //             "Tack it down",
    //         ]
    //     );
    // }

    #[test]
    pub fn leading_white_space() {
        let source = vec![
            "",
            "Shut the hatch",
            "",
            "Slide the catch",
            "",
            "Open the desk",
        ]
        .join("\n");
        assert_eq!(
            paragraphs(source.as_str()).unwrap().1,
            vec![
                "Shut the hatch",
                "Slide the catch",
                "Open the desk"
            ]
        );
    }

    #[test]
    pub fn trailing_white_space() {
        let source = vec![
            "The first scores",
            "",
            "Hang tinsel",
            "",
            "Heave the line",
            "",
            "The port side",
            "",
            "Help the weak",
            "",
        ]
        .join("\n");
        assert_eq!(
            paragraphs(source.as_str()).unwrap().1,
            vec![
                "The first scores",
                "Hang tinsel",
                "Heave the line",
                "The port side",
                "Help the weak",
            ]
        );
    }

    #[test]
    pub fn multiple_trailing_white_space() {
        let source = vec![
            "",
            "",
            "",
            "Add salt",
            "Fasten two pins",
            "",
            "Fry the egg",
            "Fly by night",
            "Waste little time",
            "",
            "",
            "Set the lamp",
            "",
            "Beat the dust",
            "",
            "",
            "",
        ]
        .join("\n");
        assert_eq!(
        paragraphs(source.as_str()).unwrap().1,
             vec![
                 "Add salt Fasten two pins",
                 "Fry the egg Fly by night Waste little time",
                 "Set the lamp",
                 "Beat the dust"
             ]
         );
    }

    #[test]
    pub fn white_space_on_blank_line() {
        let source =
            vec!["Hang tinsel", "    ", "Heave the line"]
                .join("\n");
        assert_eq!(
            paragraphs(source.as_str()).unwrap().1,
            vec!["Hang tinsel", "Heave the line",]
        );
    }
}
