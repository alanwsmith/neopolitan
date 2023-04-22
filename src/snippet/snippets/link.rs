// use crate::snippet::get_attributes::get_attributes;
use crate::snippet::snippet_enum::Snippet;
use nom::bytes::complete::is_not;
use nom::bytes::complete::tag;
use nom::multi::separated_list0;
use nom::combinator::opt;
use nom::IResult;

pub fn link(text: &str, raw_attribute_string: &str) -> Snippet {
    let attributes = get_link_attributes(raw_attribute_string);
    let mut response = String::from(r#"<a"#);
    if let Some(x) = attributes.unwrap().1 {
        response.push_str(x.as_str());
    };
    response.push_str(">");
    response.push_str(text);
    response.push_str("</a>");
    Snippet::LinkTag {
        string: Some(response.to_string()),
    }
}

pub fn get_link_attributes(source: &str) -> IResult<&str, Option<String>> {
    let mut response_string = String::from("");
    let (_, parts) = split_link_attributes(source, "|")?;
    response_string.push_str(format!(r#" href="{}""#, parts[0].trim()).as_str());
    parts.iter().skip(1).for_each(|x| {
        let (_, b) = split_link_attributes(x, ":").unwrap();
        if b.len() > 1 {
            response_string.push_str(format!(r#" {}="{}""#, b[0].trim(), b[1].trim()).as_str());
        } else {
            response_string.push_str(format!(r#" {}"#, b[0].trim()).as_str());
        }
    });

    Ok(("", Some(response_string)))
}

fn split_link_attributes<'a>(source: &'a str, separator: &'a str) -> IResult<&'a str, Vec<&'a str>> {
    let (remainder, _) = opt(tag(separator))(source)?;
    let (_, items) = separated_list0(tag(separator), is_not(separator))(remainder)?;
    Ok(("", items))
}


#[cfg(test)]
mod test {
    use crate::snippet::snippet_enum::Snippet;
    use crate::snippet::snippets::link::*;

    // #[test]
    // fn basic() {
    //     let expected = Snippet::LinkTag {
    //         string: Some("<link>Set the piece</link>".to_string()),
    //     };
    //     let results = link("Set the piece", "");
    //     assert_eq!(expected, results);
    // }

    #[test]
    fn one_attribute() {
        let expected = Snippet::LinkTag {
            string: Some(r#"<a href="https://delta.example.com/">Pile the coal</a>"#.to_string()),
        };
        let results = link("Pile the coal", r#"https://delta.example.com/"#);
        assert_eq!(expected, results);
    }

    // #[test]
    // fn two_attribute() {
    //     let expected = Snippet::LinkTag {
    //         string: Some(
    //             r#"<link id="echo" class="delta foxtrot">Raise the sail</link>"#.to_string(),
    //         ),
    //     };
    //     let results = link("Raise the sail", r#"id: echo|class: delta foxtrot"#);
    //     assert_eq!(expected, results);
    // }

    // #[test]
    // fn just_a_key() {
    //     let expected = Snippet::LinkTag {
    //         string: Some(r#"<link checked>Lift the stone</link>"#.to_string()),
    //     };
    //     let results = link("Lift the stone", r#"checked"#);
    //     assert_eq!(expected, results);
    // }
}
