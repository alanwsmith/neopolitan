use crate::section::attributes_for_section::*;
use crate::section::section::*;
use nom::bytes::complete::tag;
use nom::multi::many0;
use nom::sequence::preceded;
use nom::IResult;

pub fn youtube(source: &str) -> IResult<&str, Section> {
    let (source, att_capture) = many0(preceded(tag(">> "), section_attribute))(source).unwrap();
    let attributes = if att_capture.is_empty() {
        None
    } else {
        Some(att_capture)
    };
    Ok((source, Section::YouTubeSection { attributes }))
}

#[cfg(test)]
mod test {
    use crate::parse::parse;
    use crate::section::attributes_for_section::*;
    use crate::section::section::*;
    use crate::wrapper::wrapper::*;

    #[test]
    fn mike() {
        let lines = vec!["-> youtube", ">> asdf1234", ""].join("\n");
        let source = lines.as_str();
        let expected = Wrapper::Page {
            children: Some(vec![
                Section::YouTubeSection {
                    attributes: Some(vec![
                        (SectionAttribute::Attribute {
                            key: Some("asdf1234".to_string()),
                            value: None,
                        }),
                    ]),
                }
            ]),
        };
        let result = parse(source).unwrap().1;
        assert_eq!(expected, result);
    }
}
