use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::bytes::complete::take_until;
use nom::combinator::eof;
use nom::combinator::rest;
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
        )),
        eof,
    )(source)?;
    let block = b.0.join("");
    Ok(("", Some(block)))
}

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
    pub fn em_with_attribute() {
        assert_eq!(
            block(r#"kick <<the|em|class: alfa bravo>> ball"#),
            Ok((
                "",
                Some(String::from(r#"kick <em class="alfa bravo">the</em> ball"#))
            ))
        )
    }

    #[test]
    pub fn em_without_attributes() {
        assert_eq!(
            block(r#"kick <<the|em>> ball"#),
            Ok(("", Some(String::from(r#"kick <em>the</em> ball"#))))
        )
    }

    #[test]
    pub fn link_with_attributes() {
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

    #[test]
    pub fn link_without_attributes() {
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
    pub fn strong_tag_with_attributes() {
        assert_eq!(
            block(r#"alfa <<bravo|strong|class: delta|id: echo>> charlie"#),
            Ok((
                "",
                Some(String::from(
                    r#"alfa <strong class="delta" id="echo">bravo</strong> charlie"#
                ))
            ))
        )
    }

    #[test]
    pub fn strong_tag_without_attributes() {
        assert_eq!(
            block(r#"alfa <<bravo|strong>> charlie"#),
            Ok((
                "",
                Some(String::from(r#"alfa <strong>bravo</strong> charlie"#))
            ))
        )
    }

    //
}
