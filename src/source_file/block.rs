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
            // dbg!(&parts);
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
        "abbr" => Ok((a, format!("<abbr{}>{}</abbr>", attributes(&b, 2), b[0]))),
        "b" => Ok((a, format!("<b{}>{}</b>", attributes(&b, 2), b[0]))),
        "bdi" => Ok((a, format!("<bdi{}>{}</bdi>", attributes(&b, 2), b[0]))),
        "bdo" => Ok((a, format!("<bdo{}>{}</bdo>", attributes(&b, 2), b[0]))),
        "button" => Ok((a, format!("<button{}>{}</button>", attributes(&b, 2), b[0]))),
        "cite" => Ok((a, format!("<cite{}>{}</cite>", attributes(&b, 2), b[0]))),
        "code" => Ok((
            a,
            format!(r#"<code{}>{}</code>"#, code_attributes(&b), b[0]),
        )),
        "data" => Ok((a, format!("<data{}>{}</data>", attributes(&b, 2), b[0]))),
        "del" => Ok((a, format!("<del{}>{}</del>", attributes(&b, 2), b[0]))),
        "dfn" => Ok((a, format!("<dfn{}>{}</dfn>", attributes(&b, 2), b[0]))),
        "i" => Ok((a, format!("<i{}>{}</i>", attributes(&b, 2), b[0]))),
        "ins" => Ok((a, format!("<ins{}>{}</ins>", attributes(&b, 2), b[0]))),
        "kbd" => Ok((a, format!("<kbd{}>{}</kbd>", attributes(&b, 2), b[0]))),
        "link" => Ok((
            a,
            format!(r#"<a href="{}"{}>{}</a>"#, b[2], attributes(&b, 3), b[0]),
        )),
        "strong" => Ok((a, format!("<strong{}>{}</strong>", attributes(&b, 2), b[0]))),
        _ => Ok((a, format!(r#""#))),
    }
}

#[cfg(test)]
mod test {

    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("alfa bravo charlie", Ok(("", format!(r#"alfa bravo charlie"#))))]
    #[case("alfa <<bravo|abbr>> charlie", Ok(("", format!(r#"alfa <abbr>bravo</abbr> charlie"#))))]
    #[case("alfa <<bravo|abbr|class: delta>> charlie", Ok(("", format!(r#"alfa <abbr class="delta">bravo</abbr> charlie"#))))]
    #[case("alfa <<bravo|abbr|style: color: red;>> charlie", Ok(("", format!(r#"alfa <abbr style="color: red;">bravo</abbr> charlie"#))))]
    #[case("alfa <<bravo|b>> charlie", Ok(("", format!(r#"alfa <b>bravo</b> charlie"#))))]
    #[case("alfa <<bravo|bdi>> charlie", Ok(("", format!(r#"alfa <bdi>bravo</bdi> charlie"#))))]
    #[case("alfa <<bravo|bdo>> charlie", Ok(("", format!(r#"alfa <bdo>bravo</bdo> charlie"#))))]
    #[case("alfa <<bravo|button>> charlie", Ok(("", format!(r#"alfa <button>bravo</button> charlie"#))))]
    #[case("alfa <<bravo|cite>> charlie", Ok(("", format!(r#"alfa <cite>bravo</cite> charlie"#))))]
    #[case("alfa <<bravo|code>> charlie", Ok(("", format!(r#"alfa <code>bravo</code> charlie"#))))]
    #[case("alfa <<bravo|code|class: delta>> charlie", Ok(("", format!(r#"alfa <code class="delta">bravo</code> charlie"#))))]
    #[case("alfa <<bravo|code|rust>> charlie", Ok(("", format!(r#"alfa <code class="language-rust">bravo</code> charlie"#))))]
    #[case("alfa <<bravo|code|rust|class: delta>> charlie", Ok(("", format!(r#"alfa <code class="language-rust delta">bravo</code> charlie"#))))]
    #[case("alfa <<bravo|code|rust|class: delta|id: echo>> charlie", Ok(("", format!(r#"alfa <code id="echo" class="language-rust delta">bravo</code> charlie"#))))]
    #[case("alfa <<bravo|data>> charlie", Ok(("", format!(r#"alfa <data>bravo</data> charlie"#))))]
    #[case("alfa <<bravo|del>> charlie", Ok(("", format!(r#"alfa <del>bravo</del> charlie"#))))]
    #[case("alfa <<bravo|dfn>> charlie", Ok(("", format!(r#"alfa <dfn>bravo</dfn> charlie"#))))]
    #[case("alfa <<bravo|i>> charlie", Ok(("", format!(r#"alfa <i>bravo</i> charlie"#))))]
    #[case("alfa <<bravo|ins>> charlie", Ok(("", format!(r#"alfa <ins>bravo</ins> charlie"#))))]
    #[case("alfa <<bravo|kbd>> charlie", Ok(("", format!(r#"alfa <kbd>bravo</kbd> charlie"#))))]
    #[case("alfa <<bravo|link|https://www.example.com/>> charlie", Ok(("", format!(r#"alfa <a href="https://www.example.com/">bravo</a> charlie"#))))]
    #[case("alfa <<bravo|link|https://www.example.com/|class: delta>> charlie", Ok(("", format!(r#"alfa <a href="https://www.example.com/" class="delta">bravo</a> charlie"#))))]
    #[case("alfa <<bravo|link|https://www.example.com/|class: delta|id: echo>> charlie", Ok(("", format!(r#"alfa <a href="https://www.example.com/" class="delta" id="echo">bravo</a> charlie"#))))]
    #[case("alfa <<bravo|strong>> charlie", Ok(("", format!(r#"alfa <strong>bravo</strong> charlie"#))))]
    #[case("alfa <<bravo|strong|class: echo>> charlie", Ok(("", format!(r#"alfa <strong class="echo">bravo</strong> charlie"#))))]
    pub fn run_test(#[case] input: &str, #[case] expected: IResult<&str, String>) {
        assert_eq!(expected, block(input));
    }

    //
}
