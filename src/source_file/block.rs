use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::bytes::complete::take_until;
use nom::combinator::eof;
use nom::combinator::rest;
use nom::multi::many_till;
use nom::sequence::tuple;
use nom::IResult;
use nom::Parser;

pub fn block(source: &str) -> IResult<&str, Option<String>> {
    let (_, b) = many_till(
        alt((
            tuple((
                take_until(" <<"),
                tag(" <<"),
                take_until("|"),
                many_till(
                    tuple((tag("|"), alt((take_until("|"), take_until(">>"))))),
                    tag(">>"),
                ),
            ))
            .map(|(preface, _, text, payload)| {
                let items = payload.0;

                match items[0].1 {
                    "link" => format!(r#"{} <a href="{}">{}</a>"#, preface, items[1].1, text),
                    "strong" => format!(r#"{} <strong>{}</strong>"#, preface, text),
                    "em" => format!(r#"{} <em>{}</em>"#, preface, text),
                    _ => format!(r#"{} <a href="{}">{}</a>"#, preface, text, text),
                }

                // dbg!(&items);

                // let items = payload.0;
                // dbg!(&items);
                // format!(r#"{} <a href="{}">{}</a>"#, preface, items[2].0, items[0].0)

                // match payload.0 {
                // "em" => {
                //     format!("{}<em>{}</em>", preface, content)
                // }
                // "strong" => {
                //     format!("{}<strong>{}</strong>", preface, content)
                // }
                // "link" => {
                //     format!(
                //         r#"{}<a href="https://www.example.com/">{}</a>"#,
                //         preface, content
                //     )
                // }
                // _ => "".to_string(),

                // String::from("HHHHHHHHHHHHIIIIIIIIIITTTTTTTTTT")

                // format!("{}", String::from(items.0))
            }),
            //  rest.map(|x: &str| format!("DID NOT HIT - Remainder: {}", x)),
            rest.map(|x: &str| x.to_string()),
        )),
        eof,
    )(source)?;
    let block = b.0.join("");
    Ok(("", Some(block)))
}

// pub fn block(source: &str) -> IResult<&str, Option<String>> {
//     let (_, b) = many_till(
//         alt((
//             tuple((
//                 take_until("<<"),
//                 tag("<<"),
//                 take_until("|"),
//                 tag("|"),
//                 take_until(">>"),
//                 tag(">>"),
//             ))
//             .map(|(preface, _, content, _, tag, _)| match tag {
//                 "em" => {
//                     format!("{}<em>{}</em>", preface, content)
//                 }
//                 "strong" => {
//                     format!("{}<strong>{}</strong>", preface, content)
//                 }
//                 _ => "".to_string(),
//             }),
//             rest.map(|x: &str| x.to_string()),
//         )),
//         eof,
//     )(source)?;
//     let block = b.0.join("");
//     Ok(("", Some(block)))
// }

#[cfg(test)]
mod test {
    use crate::source_file::block::block;

    #[test]
    pub fn text_with_no_tags() {
        assert_eq!(
            block("move the radio"),
            Ok(("", Some(String::from("move the radio"))))
        )
    }

    #[test]
    pub fn text_with_strong_tag() {
        assert_eq!(
            block("wash <<the|strong>> car"),
            Ok(("", Some(String::from("wash <strong>the</strong> car"))))
        )
    }

    #[test]
    pub fn test_em_tag() {
        assert_eq!(
            block(r#"kick <<the|em>> ball"#),
            Ok(("", Some(String::from(r#"kick <em>the</em> ball"#))))
        )
    }

    #[test]
    pub fn test_link() {
        assert_eq!(
            block(r#"the <<old|link|https://www.example.com/>> rug"#),
            Ok((
                "",
                Some(String::from(
                    r#"the <a href="https://www.example.com/">old</a> rug"#
                ))
            ))
        )
    }

    #[test]
    pub fn test_extra_empty_pipe_after_link() {
        assert_eq!(
            block(r#"the <<old|link|https://www.example.com/|>> rug"#),
            Ok((
                "",
                Some(String::from(
                    r#"the <a href="https://www.example.com/">old</a> rug"#
                ))
            ))
        )
    }

    //
}
