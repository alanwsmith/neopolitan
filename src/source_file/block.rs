use nom::bytes::complete::is_not;
use nom::bytes::complete::tag;
use nom::bytes::complete::take_until;
use nom::combinator::verify;
use nom::multi::many0;
use nom::multi::separated_list1;
use nom::sequence::delimited;
use nom::sequence::tuple;
use nom::IResult;
use nom::Parser;

pub fn block(source: &str) -> IResult<&str, String> {
    let (a, b) =
        many0(tuple((take_until("<<"), neotag)).map(|(x, y)| format!("{}{}", x, y)))(source)?;
    let mut result = b.join("").to_string();
    result.push_str(a);
    Ok(("", result))
}

fn attributes(v: &Vec<&str>, position: usize) -> String {
    v.clone()
        .into_iter()
        .skip(position)
        .collect::<Vec<&str>>()
        .into_iter()
        .map(|att| {
            let parts = att.split_once(": ").unwrap();
            dbg!(&parts);
            format!(r#" {}="{}""#, parts.0, parts.1)
        })
        .collect::<Vec<String>>()
        .join("")
}

fn code_attributes(v: &Vec<&str>) -> String {
    let mut classes: Vec<String> = vec![];
    if v.len() >= 3 {
        let parts: Vec<&str> = v[2].split(": ").collect();
        if parts.len() == 1 {
            classes.push(format!("language-{}", parts[0].to_string()))
        }
    }
    let mut attributes = v
        .clone()
        .into_iter()
        .skip(2)
        .collect::<Vec<&str>>()
        .into_iter()
        .map(|att| {
            let parts: Vec<&str> = att.split(": ").collect();
            if parts.len() == 2 {
                if parts[0] == "class" {
                    classes.push(format!("{}", parts[1]));
                    format!(r#""#)
                } else {
                    format!(r#" {}="{}""#, parts[0], parts[1])
                }
            } else {
                format!("")
            }
        })
        .collect::<Vec<String>>()
        .join("");
    if classes.len() > 0 {
        attributes.push_str(r#" class=""#);
        attributes.push_str(classes.join(" ").as_str());
        attributes.push_str(r#"""#);
    }
    attributes
}

fn neotag(source: &str) -> IResult<&str, String> {
    let (a, b) = verify(
        delimited(
            tag("<<"),
            separated_list1(tag("|"), is_not("|>")),
            tag(">>"),
        ),
        |v: &Vec<&str>| v.len() > 1,
    )(source)?;
    match b[1] {
        "code" => Ok((
            a,
            format!(r#"<code{}>{}</code>"#, code_attributes(&b), b[0]),
        )),
        "link" => Ok((
            a,
            format!(r#"<a href="{}"{}>{}</a>"#, b[2], attributes(&b, 3), b[0]),
        )),
        "strong" => Ok((
            a,
            format!("<{}{}>{}</{}>", b[1], attributes(&b, 2), b[0], b[1]),
        )),
        _ => Ok((a, format!(r#""#))),
    }
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    pub fn strong_tag_no_attributes() {
        assert_eq!(
            block("<<bravo|strong>>"),
            Ok(("", format!("<strong>bravo</strong>"))),
        )
    }

    #[test]
    pub fn strong_tag_with_attributes() {
        assert_eq!(
            block("<<bravo|strong|class: echo>>"),
            Ok(("", format!(r#"<strong class="echo">bravo</strong>"#))),
        )
    }

    #[test]
    pub fn link_without_attributes() {
        assert_eq!(
            block("<<delta|link|https://www.example.com/>>"),
            Ok((
                "",
                format!(r#"<a href="https://www.example.com/">delta</a>"#)
            )),
        )
    }

    #[test]
    pub fn link_with_attributes() {
        assert_eq!(
            block("<<tango|link|https://tango.example.com/|class: whiskey>>"),
            Ok((
                "",
                format!(r#"<a href="https://tango.example.com/" class="whiskey">tango</a>"#)
            )),
        )
    }

    #[test]
    pub fn link_with_multiple_attributes() {
        assert_eq!(
            block("<<foxtrot|link|https://foxtrot.example.com/|class: november|id: zulu>>"),
            Ok((
                "",
                format!(
                    r#"<a href="https://foxtrot.example.com/" class="november" id="zulu">foxtrot</a>"#
                )
            )),
        )
    }

    #[test]
    pub fn code_without_language() {
        assert_eq!(
            block("<<tango|code>>"),
            Ok(("", format!(r#"<code>tango</code>"#))),
        )
    }

    #[test]
    pub fn code_with_language() {
        assert_eq!(
            block("<<tango|code|rust>>"),
            Ok(("", format!(r#"<code class="language-rust">tango</code>"#))),
        )
    }

    #[test]
    pub fn code_with_multiple_attributes() {
        assert_eq!(
            block("<<tango|code|class: highlighted|id: baseline>>"),
            Ok((
                "",
                format!(r#"<code id="baseline" class="highlighted">tango</code>"#)
            )),
        )
    }

    #[test]
    pub fn code_with_language_and_class_attributes() {
        assert_eq!(
            block("<<tango|code|rust|class: highlighted|id: baseline>>"),
            Ok((
                "",
                format!(r#"<code id="baseline" class="language-rust highlighted">tango</code>"#)
            )),
        )
    }

    #[test]
    pub fn no_tags() {
        assert_eq!(
            block("light the candle"),
            Ok(("", format!(r#"light the candle"#)))
        )
    }

    #[test]
    pub fn prelude_text_with_tags() {
        assert_eq!(
            block("light <<the|strong>> candle"),
            Ok(("", format!(r#"light <strong>the</strong> candle"#)))
        )
    }

    #[test]
    pub fn solo_colons_in_attributes() {
        assert_eq!(
            block("alfa <<bravo|strong|style: color: red;>> charlie"),
            Ok((
                "",
                format!(r#"alfa <strong style="color: red;">bravo</strong> charlie"#)
            ))
        )
    }

    //
}
