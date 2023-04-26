use crate::attribute::*;
use crate::content::content::Content;
use crate::content::neo_tag::split_attribute;
use nom::bytes::complete::is_not;
use nom::bytes::complete::tag;
use nom::multi::separated_list0;
use nom::IResult;

pub fn link_shorthand_attributes(source: &str) -> IResult<&str, Option<Vec<Attribute>>> {
    let (_, parts) = separated_list0(tag("|"), is_not("|"))(source)?;
    let mut atts: Vec<Attribute> = vec![Attribute::Basic {
        key: Some(parts[0].to_string()),
        value: None,
    }];
    if parts.len() > 1 {
        parts.iter().skip(1).for_each(|a| {
            atts.push(split_attribute(a).unwrap().1.unwrap());
        });
    }
    Ok(("", Some(atts)))
}

pub fn link_shorthand<'a>(
    source: (&'a str, &'a str, &'a str, &'a str, &'a str),
) -> IResult<&'a str, Content> {
    Ok((
        "",
        Content::LinkShorthand {
            attributes: link_shorthand_attributes(source.3).unwrap().1,
            //attributes: None,
            text: Some(source.1.to_string()),
        },
    ))
}

#[cfg(test)]
mod test {
    use crate::attribute::*;
    use crate::block::block::*;
    use crate::content::content::*;
    use crate::parse::parse;
    use crate::section::section::*;
    use crate::wrapper::wrapper::*;

    #[test]
    fn basic_link_shorthand() {
        let lines = vec!["-> p", "", ">tango uniform>https://alfa.example.com/>"].join("\n");
        let source = lines.as_str();
        let expected = Wrapper::Page {
            children: Some(vec![Section::Paragraphs {
                attributes: None,
                children: Some(vec![Block::P {
                    children: Some(vec![Content::LinkShorthand {
                        attributes: Some(vec![Attribute::Basic {
                            key: Some("https://alfa.example.com/".to_string()),
                            value: None,
                        }]),
                        text: Some("tango uniform".to_string()),
                    }]),
                }]),
            }]),
        };
        let result = parse(source).unwrap().1;
        assert_eq!(expected, result);
    }
}
