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
                // let attributes = r#" class="highlighted""#;

                let attributes: String = items
                    .clone()
                    .into_iter()
                    .skip(2)
                    .map(|x: (&str, &str)| {
                        let parts: Vec<&str> = x.1.split(": ").collect();
                        format!(r#" {}="{}""#, parts[0], parts[1])
                    })
                    .collect();
                // &things.skip(2);

                // dbg!(&items);
                // dbg!(&things);

                match items[0].1 {
                    "link" => format!(
                        r#"{} <a href="{}"{}>{}</a>"#,
                        preface, items[1].1, attributes, text
                    ),
                    "strong" => format!(r#"{} <strong>{}</strong>"#, preface, text),
                    "em" => format!(r#"{} <em>{}</em>"#, preface, text),
                    _ => format!(r#"{} <a href="{}">{}</a>"#, preface, text, text),
                }
            }),
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
//                 take_until(" <<"),
//                 tag(" <<"),
//                 take_until("|"),
//                 many_till(
//                     tuple((tag("|"), alt((take_until("|"), take_until(">>"))))),
//                     tag(">>"),
//                 ),
//             ))
//             .map(|(preface, _, text, payload)| {
//                 let items = payload.0;
//                 match items[0].1 {
//                     "link" => format!(r#"{} <a href="{}">{}</a>"#, preface, items[1].1, text),
//                     "strong" => format!(r#"{} <strong>{}</strong>"#, preface, text),
//                     "em" => format!(r#"{} <em>{}</em>"#, preface, text),
//                     _ => format!(r#"{} <a href="{}">{}</a>"#, preface, text, text),
//                 }
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
    pub fn basic_strong_tag() {
        assert_eq!(
            block("wash <<the|strong>> car"),
            Ok(("", Some(String::from("wash <strong>the</strong> car"))))
        )
    }

    #[test]
    pub fn basic_em_tag() {
        assert_eq!(
            block(r#"kick <<the|em>> ball"#),
            Ok(("", Some(String::from(r#"kick <em>the</em> ball"#))))
        )
    }

    #[test]
    pub fn basic_link() {
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

    // NOTE: Not going to do this. An empyt pipe as the last character
    // is not valid
    // #[test]
    // pub fn extra_empty_pipe_after_link() {
    //     assert_eq!(
    //         block(r#"the <<old|link|https://www.example.com/|>> rug"#),
    //         Ok((
    //             "",
    //             Some(String::from(
    //                 r#"the <a href="https://www.example.com/">old</a> rug"#
    //             ))
    //         ))
    //     )
    // }

    #[test]
    pub fn link_with_attribute() {
        assert_eq!(
            block(r#"break <<the|link|https://www.example.com/|class: highlighted>> glass"#),
            Ok((
                "",
                Some(String::from(
                    r#"break <a href="https://www.example.com/" class="highlighted">the</a> glass"#
                ))
            ))
        )
    }

    //
}
