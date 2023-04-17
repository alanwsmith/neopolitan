use crate::attribute::*;
use crate::content::content::Content;
use nom::branch::alt;
use nom::bytes::complete::is_not;
use nom::bytes::complete::tag;
use nom::bytes::complete::tag_no_case;
use nom::bytes::complete::take_until;
use nom::character::complete::multispace0;
use nom::combinator::opt;
use nom::combinator::rest;
use nom::multi::separated_list0;
use nom::sequence::tuple;
use nom::IResult;
use nom::Parser;

pub fn neo_tag_attributes(source: &str) -> IResult<&str, Option<Vec<Attribute>>> {
    let (_, parts) = separated_list0(tag("|"), is_not("|"))(source)?;
    if parts.len() > 1 {
        let attributes: Option<Vec<Attribute>> = Some(
            parts
                .iter()
                .skip(1)
                .map(|a| attribute(a).unwrap().1)
                .collect(),
        );
        Ok(("", attributes))
    } else {
        Ok(("", None))
    }
}

pub fn split_attribute(source: &str) -> IResult<&str, Option<Attribute>> {
    let (remainder, _) = opt(tag(":"))(source)?;
    let (_, items) = separated_list0(tag(":"), is_not(":"))(remainder)?;
    if items.len() > 1 {
        Ok((
            "",
            Some(Attribute::Basic {
                key: Some(items[0].trim().to_string()),
                value: Some(items[1].trim().to_string()),
            }),
        ))
    } else {
        Ok(("", None))
    }
}

pub fn neo_tag_link_attributes(source: &str) -> IResult<&str, Option<Vec<Attribute>>> {
    let (_, parts) = separated_list0(tag("|"), is_not("|"))(source)?;
    let mut atts: Vec<Attribute> = vec![Attribute::Basic {
        key: Some(parts[1].to_string()),
        value: None,
    }];
    if parts.len() > 2 {
        parts.iter().skip(2).for_each(|a| {
            atts.push(split_attribute(a).unwrap().1.unwrap());
        });
    }
    Ok(("", Some(atts)))
}

pub fn neo_tag<'a>(source: (&'a str, &'a str, &'a str)) -> IResult<&'a str, Content> {
    let (_, parts) = tuple((take_until("|"), tag("|"), rest))(source.1)?;
    let (a, b) = alt((
        tuple((multispace0, tag_no_case("b"), multispace0)).map(|_| Content::B {
            attributes: neo_tag_attributes(parts.2).unwrap().1,
            text: Some(parts.0.to_string()),
        }),
        tuple((multispace0, tag_no_case("code"), multispace0)).map(|_| Content::Code {
            attributes: neo_tag_attributes(parts.2).unwrap().1,
            text: Some(parts.0.to_string()),
        }),
        tuple((multispace0, tag_no_case("em"), multispace0)).map(|_| Content::Em {
            attributes: neo_tag_attributes(parts.2).unwrap().1,
            text: Some(parts.0.to_string()),
        }),
        tuple((multispace0, tag_no_case("i"), multispace0)).map(|_| Content::I {
            attributes: neo_tag_attributes(parts.2).unwrap().1,
            text: Some(parts.0.to_string()),
        }),
        tuple((multispace0, tag_no_case("kbd"), multispace0)).map(|_| Content::Kbd {
            attributes: neo_tag_attributes(parts.2).unwrap().1,
            text: Some(parts.0.to_string()),
        }),
        tuple((multispace0, tag_no_case("link"), multispace0)).map(|_| Content::Link {
            attributes: neo_tag_link_attributes(parts.2).unwrap().1,
            text: Some(parts.0.to_string()),
        }),
        tuple((multispace0, tag_no_case("span"), multispace0)).map(|_| Content::Span {
            attributes: neo_tag_attributes(parts.2).unwrap().1,
            text: Some(parts.0.to_string()),
        }),
        tuple((multispace0, tag_no_case("strike"), multispace0)).map(|_| Content::Strike {
            attributes: neo_tag_attributes(parts.2).unwrap().1,
            text: Some(parts.0.to_string()),
        }),
        tuple((multispace0, tag_no_case("strong"), multispace0)).map(|_| Content::Strong {
            attributes: neo_tag_attributes(parts.2).unwrap().1,
            text: Some(parts.0.to_string()),
        }),
        tuple((multispace0, tag_no_case("sub"), multispace0)).map(|_| Content::Sub {
            attributes: neo_tag_attributes(parts.2).unwrap().1,
            text: Some(parts.0.to_string()),
        }),
        tuple((multispace0, tag_no_case("sup"), multispace0)).map(|_| Content::Sup {
            attributes: neo_tag_attributes(parts.2).unwrap().1,
            text: Some(parts.0.to_string()),
        }),
        tuple((multispace0, tag_no_case("u"), multispace0)).map(|_| Content::U {
            attributes: neo_tag_attributes(parts.2).unwrap().1,
            text: Some(parts.0.to_string()),
        }),
    ))(parts.2)?;
    Ok((a, b))
}

#[cfg(test)]
mod test {

    use crate::attribute::*;
    use crate::content::content::*;
    use crate::content::neo_tag::neo_tag;

    #[test]
    fn b() {
        let source = ("", "alfa|b", "");
        let expected = Content::B {
            attributes: None,
            text: Some("alfa".to_string()),
        };
        let result = neo_tag(source);
        assert_eq!(expected, result.unwrap().1);
    }

    #[test]
    fn b_with_one_attribute() {
        let source = ("", "bravo|b|class: charlie", "");
        let expected = Content::B {
            attributes: Some(vec![Attribute::Basic {
                key: Some("class".to_string()),
                value: Some("charlie".to_string()),
            }]),
            text: Some("bravo".to_string()),
        };
        let result = neo_tag(source);
        assert_eq!(expected, result.unwrap().1);
    }

    #[test]
    fn b_with_two_attributes() {
        let source = ("", "charlie|b|class: delta|id: echo", "");
        let expected = Content::B {
            attributes: Some(vec![
                Attribute::Basic {
                    key: Some("class".to_string()),
                    value: Some("delta".to_string()),
                },
                Attribute::Basic {
                    key: Some("id".to_string()),
                    value: Some("echo".to_string()),
                },
            ]),
            text: Some("charlie".to_string()),
        };
        let result = neo_tag(source);
        assert_eq!(expected, result.unwrap().1);
    }

    #[test]
    fn code() {
        let source = ("", "foxtrot|code", "");
        let expected = Content::Code {
            attributes: None,
            text: Some("foxtrot".to_string()),
        };
        let result = neo_tag(source);
        assert_eq!(expected, result.unwrap().1);
    }

    #[test]
    fn code_with_single_key_for_language() {
        let source = ("", "golf|code|rust", "");
        let expected = Content::Code {
            attributes: Some(vec![Attribute::Basic {
                key: Some("rust".to_string()),
                value: None,
            }]),
            text: Some("golf".to_string()),
        };
        let result = neo_tag(source);
        assert_eq!(expected, result.unwrap().1);
    }

    #[test]
    fn code_with_regular_attributes() {
        let source = ("", "hotel|code|class: india|id: juliette", "");
        let expected = Content::Code {
            text: Some("hotel".to_string()),
            attributes: Some(vec![
                Attribute::Basic {
                    key: Some("class".to_string()),
                    value: Some("india".to_string()),
                },
                Attribute::Basic {
                    key: Some("id".to_string()),
                    value: Some("juliette".to_string()),
                },
            ]),
        };
        let result = neo_tag(source);
        assert_eq!(expected, result.unwrap().1);
    }

    #[test]
    fn em() {
        let source = ("", "kilo|em", "");
        let expected = Content::Em {
            text: Some("kilo".to_string()),
            attributes: None,
        };
        let result = neo_tag(source);
        assert_eq!(expected, result.unwrap().1);
    }

    #[test]
    fn i() {
        let source = ("", "lima|i", "");
        let expected = Content::I {
            text: Some("lima".to_string()),
            attributes: None,
        };
        let result = neo_tag(source);
        assert_eq!(expected, result.unwrap().1);
    }

    #[test]
    fn kbd() {
        let source = ("", "mike|kbd", "");
        let expected = Content::Kbd {
            text: Some("mike".to_string()),
            attributes: None,
        };
        let result = neo_tag(source);
        assert_eq!(expected, result.unwrap().1);
    }

    #[test]
    fn link() {
        let source = ("", "sierra|link|https://www.example.com/", "");
        let expected = Content::Link {
            text: Some("sierra".to_string()),
            attributes: Some(vec![Attribute::Basic {
                key: Some("https://www.example.com/".to_string()),
                value: None,
            }]),
        };
        let result = neo_tag(source);
        assert_eq!(expected, result.unwrap().1);
    }

    #[test]
    fn link_with_attributes() {
        let source = ("", "tango|link|https://www.example.com/|class: random", "");
        let expected = Content::Link {
            text: Some("tango".to_string()),
            attributes: Some(vec![
                Attribute::Basic {
                    key: Some("https://www.example.com/".to_string()),
                    value: None,
                },
                Attribute::Basic {
                    key: Some("class".to_string()),
                    value: Some("random".to_string()),
                },
            ]),
        };
        let result = neo_tag(source);
        assert_eq!(expected, result.unwrap().1);
    }

    #[test]
    fn span() {
        let source = ("", "november|span", "");
        let expected = Content::Span {
            text: Some("november".to_string()),
            attributes: None,
        };
        let result = neo_tag(source);
        assert_eq!(expected, result.unwrap().1);
    }

    #[test]
    fn strike() {
        let source = ("", "oscar|strike", "");
        let expected = Content::Strike {
            text: Some("oscar".to_string()),
            attributes: None,
        };
        let result = neo_tag(source);
        assert_eq!(expected, result.unwrap().1);
    }

    #[test]
    fn strong() {
        let source = ("", "bravo|strong|class: delta", "");
        let expected = Content::Strong {
            attributes: Some(vec![Attribute::Basic {
                key: Some("class".to_string()),
                value: Some("delta".to_string()),
            }]),
            text: Some("bravo".to_string()),
        };
        let result = neo_tag(source);
        assert_eq!(expected, result.unwrap().1);
    }

    #[test]
    fn sub() {
        let source = ("", "papa|sub", "");
        let expected = Content::Sub {
            text: Some("papa".to_string()),
            attributes: None,
        };
        let result = neo_tag(source);
        assert_eq!(expected, result.unwrap().1);
    }

    #[test]
    fn sup() {
        let source = ("", "quebec|sup", "");
        let expected = Content::Sup {
            text: Some("quebec".to_string()),
            attributes: None,
        };
        let result = neo_tag(source);
        assert_eq!(expected, result.unwrap().1);
    }

    #[test]
    fn u() {
        let source = ("", "romeo|u", "");
        let expected = Content::U {
            text: Some("romeo".to_string()),
            attributes: None,
        };
        let result = neo_tag(source);
        assert_eq!(expected, result.unwrap().1);
    }
}
