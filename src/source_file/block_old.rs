use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::bytes::complete::take_until;
// use nom::combinator::eof;
use nom::combinator::rest;
use nom::multi::many0;
use nom::multi::many_till;
use nom::sequence::tuple;
use nom::IResult;
use nom::Parser;

pub fn attributes(source: Vec<(&str, &str)>, skip: usize) -> String {
    source
        .into_iter()
        .skip(skip)
        .map(|x: (&str, &str)| {
            let parts: Vec<&str> = x.1.split(": ").collect();
            format!(r#" {}="{}""#, parts[0], parts[1])
        })
        .collect()
}

pub fn block(source: &str) -> IResult<&str, Option<String>> {
    let (_, b) = many0(alt((
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
                "abbr" => {
                    format!(
                        r#"{} <abbr{}>{}</abbr>"#,
                        preface,
                        attributes(items.clone(), 1),
                        text
                    )
                }
                "b" => {
                    format!(
                        r#"{} <b{}>{}</b>"#,
                        preface,
                        attributes(items.clone(), 1),
                        text
                    )
                }
                "button" => {
                    format!(
                        r#"{} <button{}>{}</button>"#,
                        preface,
                        attributes(items.clone(), 1),
                        text
                    )
                }
                "data" => {
                    format!(
                        r#"{} <data{}>{}</data>"#,
                        preface,
                        attributes(items.clone(), 1),
                        text
                    )
                }
                "del" => {
                    format!(
                        r#"{} <del{}>{}</del>"#,
                        preface,
                        attributes(items.clone(), 1),
                        text
                    )
                }
                "link" => {
                    format!(
                        r#"{} <a href="{}"{}>{}</a>"#,
                        preface,
                        items[1].1,
                        attributes(items.clone(), 2),
                        text
                    )
                }
                "em" => {
                    format!(
                        r#"{} <em{}>{}</em>"#,
                        preface,
                        attributes(items.clone(), 1),
                        text
                    )
                }
                "strong" => {
                    format!(
                        r#"{} <strong{}>{}</strong>"#,
                        preface,
                        attributes(items.clone(), 1),
                        text
                    )
                }
                _ => format!(r#"{} <a href="{}">{}</a>"#, preface, text, text),
            }
        }),
        rest.map(|x: &str| x.to_string()),
    )))(source)?;
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
//                     "abbr" => {
//                         format!(
//                             r#"{} <abbr{}>{}</abbr>"#,
//                             preface,
//                             attributes(items.clone(), 1),
//                             text
//                         )
//                     }
//                     "b" => {
//                         format!(
//                             r#"{} <b{}>{}</b>"#,
//                             preface,
//                             attributes(items.clone(), 1),
//                             text
//                         )
//                     }
//                     "button" => {
//                         format!(
//                             r#"{} <button{}>{}</button>"#,
//                             preface,
//                             attributes(items.clone(), 1),
//                             text
//                         )
//                     }
//                     "data" => {
//                         format!(
//                             r#"{} <data{}>{}</data>"#,
//                             preface,
//                             attributes(items.clone(), 1),
//                             text
//                         )
//                     }
//                     "del" => {
//                         format!(
//                             r#"{} <del{}>{}</del>"#,
//                             preface,
//                             attributes(items.clone(), 1),
//                             text
//                         )
//                     }
//                     "link" => {
//                         format!(
//                             r#"{} <a href="{}"{}>{}</a>"#,
//                             preface,
//                             items[1].1,
//                             attributes(items.clone(), 2),
//                             text
//                         )
//                     }
//                     "em" => {
//                         format!(
//                             r#"{} <em{}>{}</em>"#,
//                             preface,
//                             attributes(items.clone(), 1),
//                             text
//                         )
//                     }
//                     "strong" => {
//                         format!(
//                             r#"{} <strong{}>{}</strong>"#,
//                             preface,
//                             attributes(items.clone(), 1),
//                             text
//                         )
//                     }
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
    use nom::error::Error;
    use nom::Err;

    use rstest::rstest;

    #[rstest]
    #[case("move the radio", Ok(("", Some(String::from("move the radio")))))]
    #[case(r#"alfa <<bravo|abbr>> charlie"#, Ok(("", Some(String::from(r#"alfa <abbr>bravo</abbr> charlie"#)))))]
    #[case(r#"delt <<echoo|abbr|class: foxtrot>> tango"#, Ok(("", Some(String::from(r#"delt <abbr class="foxtrot">echoo</abbr> tango"#)))))]
    #[case(r#"alfa <<bravo|em>> charlie"#, Ok(("", Some(String::from(r#"alfa <em>bravo</em> charlie"#)))))]
    #[case(r#"delt <<echoo|em|class: foxtrot>> tango"#, Ok(("", Some(String::from(r#"delt <em class="foxtrot">echoo</em> tango"#)))))]
    #[case(r#"alfa <<bravo|link|https://www.example.com/>> charlie"#, Ok(("", Some(String::from(r#"alfa <a href="https://www.example.com/">bravo</a> charlie"#)))))]
    #[case(r#"delt <<echoo|link|https://www.example.com/|class: foxtrot>> tango"#, Ok(("", Some(String::from(r#"delt <a href="https://www.example.com/" class="foxtrot">echoo</a> tango"#)))))]
    #[case(r#"alfa <<bravo|strong>> charlie"#, Ok(("", Some(String::from(r#"alfa <strong>bravo</strong> charlie"#)))))]
    #[case(r#"delt <<echoo|strong|class: foxtrot>> tango"#, Ok(("", Some(String::from(r#"delt <strong class="foxtrot">echoo</strong> tango"#)))))]
    #[case(r#"alfa <<bravo|b>> charlie"#, Ok(("", Some(String::from(r#"alfa <b>bravo</b> charlie"#)))))]
    #[case(r#"delt <<echoo|b|class: foxtrot>> tango"#, Ok(("", Some(String::from(r#"delt <b class="foxtrot">echoo</b> tango"#)))))]
    #[case(r#"alfa <<bravo|button>> charlie"#, Ok(("", Some(String::from(r#"alfa <button>bravo</button> charlie"#)))))]
    #[case(r#"delt <<echoo|button|class: foxtrot>> tango"#, Ok(("", Some(String::from(r#"delt <button class="foxtrot">echoo</button> tango"#)))))]
    #[case(r#"alfa <<bravo|data>> charlie"#, Ok(("", Some(String::from(r#"alfa <data>bravo</data> charlie"#)))))]
    #[case(r#"delt <<echoo|data|class: foxtrot>> tango"#, Ok(("", Some(String::from(r#"delt <data class="foxtrot">echoo</data> tango"#)))))]
    #[case(r#"alfa <<bravo|data>> charlie"#, Ok(("", Some(String::from(r#"alfa <data>bravo</data> charlie"#)))))]
    #[case(r#"delt <<echoo|data|class: foxtrot>> tango"#, Ok(("", Some(String::from(r#"delt <data class="foxtrot">echoo</data> tango"#)))))]
    #[case(r#"alfa <<bravo|del>> charlie"#, Ok(("", Some(String::from(r#"alfa <del>bravo</del> charlie"#)))))]
    #[case(r#"delt <<echoo|del|class: foxtrot>> tango"#, Ok(("", Some(String::from(r#"delt <del class="foxtrot">echoo</del> tango"#)))))]
    fn block_test(
        #[case] input: &str,
        #[case] expected: Result<(&str, Option<String>), Err<Error<&str>>>,
    ) {
        assert_eq!(expected, block(input))
    }

    #[test]
    fn multiple_tags() {
        assert_eq!(
            block(r#"the <<quick|strong>> brown <<fox|link|https://www.example.com/>>"#),
            Ok((
                "",
                Some(String::from(
                    r#"the <strong>quick</strong> brown <a href="https://www.example.com/">fox</a>"#
                ))
            ))
        )
    }

    //
}
