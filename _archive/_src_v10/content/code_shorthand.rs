use crate::attribute::*;
use crate::content::content::Content;
use nom::bytes::complete::is_not;
use nom::bytes::complete::tag;
use nom::multi::separated_list0;
use nom::IResult;

pub fn code_shorthand<'a>(
    source: (&'a str, &'a str, &'a str, &'a str, &'a str),
) -> IResult<&'a str, Content> {
    let (_, items) = separated_list0(tag("|"), is_not("|"))(source.3)?;
    if items.len() > 0 {
        let attributes: Option<Vec<Attribute>> =
            Some(items.iter().map(|a| attribute(a).unwrap().1).collect());
        Ok((
            "",
            Content::CodeShorthand {
                attributes,
                text: Some(source.1.to_string()),
            },
        ))
    } else {
        Ok((
            "",
            Content::CodeShorthand {
                attributes: None,
                text: Some(source.1.to_string()),
            },
        ))
    }
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
    fn basic_code_shorthand() {
        let lines = vec!["-> p", "", "`tango uniform``", "`alfa`rust`"].join("\n");
        let source = lines.as_str();
        let expected = Wrapper::Page {
            children: Some(vec![Section::Paragraphs {
                attributes: None,
                children: Some(vec![Block::P {
                    children: Some(vec![
                        Content::CodeShorthand {
                            attributes: None,
                            text: Some("tango uniform".to_string()),
                        },
                        Content::Space,
                        Content::CodeShorthand {
                            attributes: Some(vec![Attribute::Basic {
                                key: Some("rust".to_string()),
                                value: None,
                            }]),
                            text: Some("alfa".to_string()),
                        },
                    ]),
                }]),
            }]),
        };
        let result = parse(source).unwrap().1;
        assert_eq!(expected, result);
    }
}
